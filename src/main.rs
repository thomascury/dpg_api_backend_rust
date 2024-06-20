mod word_list;
use word_list::get_beale_word;
mod config;
use config::Configuration;

use lambda_http::{run, service_fn, Body, Error as LambdaError, Request, RequestExt, Response};
use rand::prelude::*;
#[macro_use] extern crate log;
extern crate core;

use env_logger;
use core::fmt;

const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SPECIALS: &str = r##"~!#$%^&*()-=+[]{}:;\"'<>?/"##;

fn is_special (c: char) -> bool {
    return SPECIALS.contains(c);
}

fn is_decimal (c: char) -> bool {
    return c.is_digit(10);
}

struct Policy {
    name: &'static str,
    chars: &'static str,
    is_active: bool,
    validate: &'static dyn Fn(char) -> bool,
    enforce: &'static dyn Fn(Self, Vec<String>, &str) -> Vec<String>,
}

impl fmt::Display for Policy {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (is_active: {})", self.name, self.is_active)?;
        Ok(())
    }
}

fn check_string (string: &String, check_function: &dyn Fn(char) -> bool) -> bool {
    let mut check_succeeded = false;
    for c in string.chars(){
        if check_function(c){
            check_succeeded = true;
            break;
        }
    }
    return check_succeeded;
}

fn enforce_policy (policy: Policy, mut words: Vec<String>, separator: &str) -> Vec<String> {
    let passphrase: String = words.join(separator);
    if policy.is_active {
        if !check_string(&passphrase, policy.validate) {
            debug!("Check for {} failed", policy);
            let mut rng = thread_rng();
            let word_index: usize = rng.gen_range(0..words.len());
            let char_index: usize = rng.gen_range(0..words[word_index].len());
            let insert_char_index: usize = rng.gen_range(0..policy.chars.len());
            let chars_vec: Vec<char> = policy.chars.chars().collect();
            let insert_char = chars_vec[insert_char_index];
            debug!(" > Adding char '{}' to word '{}' at position {}", insert_char, words[word_index], char_index);
            words[word_index].replace_range(char_index..char_index + 1, &insert_char.to_string());
            debug!(" > New word is '{}'", words[word_index]);
        }
    }
    return words;
}

fn initialize_policies (configuration: &Configuration) -> Vec<Policy> {
    let mut policies: Vec<Policy> = Vec::with_capacity(4);
    let lowercase_policy = Policy {
        name: &"lowercase policy",
        chars: &LOWER,
        is_active: configuration.get_use_lower(),
        validate: &char::is_lowercase,
        enforce: &enforce_policy,
    };
    policies.push(lowercase_policy);
    let uppercase_policy = Policy {
        name: &"uppercase policy",
        chars: &UPPER,
        is_active: configuration.get_use_upper(),
        validate: &char::is_uppercase,
        enforce: &enforce_policy,
    };
    policies.push(uppercase_policy);
    let digit_policy = Policy {
        name: &"digit policy",
        chars: &DIGITS,
        is_active: configuration.get_use_digit(),
        validate: &is_decimal,
        enforce: &enforce_policy,
    };
    policies.push(digit_policy);
    let special_policy = Policy {
        name: &"special policy",
        chars: &SPECIALS,
        is_active: configuration.get_use_special(),
        validate: &is_special,
        enforce: &enforce_policy,
    };
    policies.push(special_policy);
    return policies;
}

fn get_word (capitalized: bool) -> String {
    let mut rng = thread_rng();
    let mut throws: Vec<char> = Vec::with_capacity(5);

    for _ in 0..throws.capacity() {
        throws.push(char::from_digit(rng.gen_range(1..=6), 10).expect("Not a digit"));
    }

    let throws_as_str: String = throws.into_iter().collect();
    let mut word = get_beale_word(&throws_as_str).unwrap_or("Undefined").to_string();
    debug!("Beale word for key '{}' is '{}'", throws_as_str, word);

    if capitalized {
        word = format!("{}{word}", word.remove(0).to_uppercase());
        debug!("Capitalized word to '{}'", word);
    }

    return word;
}

fn create_passphrase (configuration: Configuration) -> Result<String, &'static str> {
    
    let mut words: Vec<String> = Vec::with_capacity(configuration.get_word_count().into());
    for _ in 0_u8..configuration.get_word_count() {
        words.push(get_word(configuration.get_capitalize()));
    }

    let policies = initialize_policies(&configuration);
    
    // Ensure specified conditions
    for policy in policies {
        if policy.is_active {
            words = (policy.enforce)(policy, words.clone(), configuration.get_separator());
        }
    }
    
    let passphrase: String = words.join(configuration.get_separator());
    info!("Generated passphrase is '{}'", passphrase);

    return Ok(passphrase);
}

fn generate_response (status: u16, body: &str) -> Result<Response<Body>, LambdaError> {
    Ok(Response::builder()
        .status(status)
        .header("content-type", "text/html")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "OPTIONS,POST,GET")
        .body(body.into())
        .map_err(Box::new)?)
}

async fn function_handler(event: Request) -> Result<Response<Body>, LambdaError> {
    // AWS Runtime can ignore Stage Name passed from json event
    // https://github.com/awslabs/aws-lambda-rust-runtime/issues/782
    std::env::set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

    // Set default values
    let mut gen_parameters = Configuration::init();

    // Get parameters
    match event.query_string_parameters_ref() {
        Some(query_parameters) => {
            if query_parameters.is_empty() {
                return generate_response(400, "QueryMap is empty");
            }
            // Debug print of query parameters
            for (key, value) in query_parameters.iter().into_iter() {
                println!("Parameter: '{}', value:'{}'", key, value)
            }

            // Initiating logging facilities
            fn get_loglevel (loglevel_parameter: Option<&str>) -> &str {
                match loglevel_parameter {
                    Some(level) => {
                        println!("Found loglevel '{}'", level);
                        return level;
                    },
                    None => {
                        println!("Could not find loglevel parameter in QueryMap, setting to default value 'info'");
                        return "info";
                    }
                }
            }
            // Using standard logging
            let level = get_loglevel(query_parameters.first("loglevel"));
            std::env::set_var("RUST_LOG", level);
            let _ = env_logger::try_init();
            debug!("Initialized logging facilities with max logging level set to '{}'", level)
        },
        None => return generate_response(400, "Missing QueryMap"),
    }
    if let Some(query_parameters) = event.query_string_parameters_ref() {
        // Parse parameters
        for field in gen_parameters.field_list.clone() {
            match field {
                "word_count" => gen_parameters.set_word_count(query_parameters.first("nbwords")),
                "separator" => gen_parameters.set_separator(query_parameters.first("separator")),
                "capitalize" => gen_parameters.set_capitalize(query_parameters.first("capitalize")),
                "use_lower" => gen_parameters.set_use_lower(query_parameters.first("lower")),
                "use_upper" => gen_parameters.set_use_upper(query_parameters.first("upper")),
                "use_digit" => gen_parameters.set_use_digit(query_parameters.first("number")),
                "use_special" => gen_parameters.set_use_special(query_parameters.first("special")),
                _ => (),
            }
        };
    }
    debug!("{}", gen_parameters);

    match create_passphrase(gen_parameters) {
        Ok(passphrase) => generate_response(200, passphrase.as_str()),
        Err(err) => generate_response(500, err.into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    run(service_fn(function_handler)).await
}
