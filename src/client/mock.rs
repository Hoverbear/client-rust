#![feature(core_intrinsics)]
#![feature(async_await)]

#[macro_use]
extern crate clap;
extern crate rustyline;

use clap::{App, ArgMatches};
use std::path::PathBuf;
use tikv_client::{*, raw::*};
use std::intrinsics::atomic_max_rel;
use std::cmp::min;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::Read;

#[derive(Debug)]
pub struct TlsOptions {
    pub ca_path: PathBuf,
    pub cert_path: PathBuf,
    pub key_path: PathBuf
}

#[derive(Debug)]
pub struct ClientOptions {
    pub logging: String,
    pub minify: bool,
    pub mode: String,
    pub output_durations: bool,
    pub key_encoding: String,
    pub value_encoding: String
}

/*#[derive(Debug, Deserialize)]
pub struct TomlConfig {
    endpoints: Vec<String>,
    tls_opts: TlsOptions,
    client_opts: ClientOptions
}*/

impl Default for ClientOptions {
    fn default() -> ClientOptions {
        ClientOptions {
            logging: String::from("info"),
            minify: false,
            mode: String::from("transaction"),
            output_durations: true,
            key_encoding: String::from("utf-8"),
            value_encoding: String::from("utf-8")
        }
    }
}

fn generate_config_from_arguments(endpoints: Vec<&str>, matches: &ArgMatches) -> Config {
    if matches.is_present("key_path") {
        let tls_opts: TlsOptions = TlsOptions {
            ca_path: matches.value_of("ca_path").map(PathBuf::from).unwrap(),
            cert_path: matches.value_of("cert_path").map(PathBuf::from).unwrap(),
            key_path: matches.value_of("key_path").map(PathBuf::from).unwrap()
        };

        println!("{:?}", tls_opts);

        Config::new(endpoints).with_security(tls_opts.ca_path, tls_opts.cert_path, tls_opts.key_path)
    } else {
        Config::new(endpoints)
    }
}

fn create_client_opts_from_arguments(matches: &ArgMatches) -> ClientOptions {
    ClientOptions {
        logging: if let Some(logging) = matches.value_of("logging") {
            String::from(logging)
        } else {
            String::from("info")
        },
        minify: if matches.is_present("minify") {
            true
        } else {
            false
        },
        mode: if let Some(mode) = matches.value_of("mode") {
            String::from(mode)
        } else {
            String::from("transaction")
        },
        output_durations: if matches.is_present("output_durations") {
            false
        } else {
            true
        },
        key_encoding: if let Some(key_encoding) = matches.value_of("key_encoding") {
            String::from(key_encoding)
        } else {
            String::from("utf-8")
        },
        value_encoding: if let Some(value_encoding) = matches.value_of("value_encoding") {
            String::from(value_encoding)
        } else {
            String::from("utf-8")
        }
    }
}

/*fn generate_stuff_from_file() {
    let mut config_toml = String::new();

    let mut file = match File::open("example.toml") {
        Ok(file) => file,
        Err(_)  => {
            // Return default?
        }
    };

    file.read_to_string(&mut config_toml)
        .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

    let decoded: TomlConfig = toml::from_str(&*config_toml).unwrap();
    println!("{:#?}", decoded);
}*/

#[cfg(feature = "yaml")]
fn main() {
    // Load yml
    let yaml = load_yaml!("options.yaml");
    let matches: ArgMatches = App::from_yaml(yaml).get_matches();

    // Read pd_endpoints from command line arguments or set default localhost
    let endpoints: Vec<_> = if let Some(pd_endpoints) = matches.values_of("pd_endpoints") {
        pd_endpoints.collect()
    } else {
        let mut pde: Vec<&str> = Vec::new();
        pde.push("localhost");
        pde
    };

    // By settings if key_path is present then cert_path and ca_path are also present
    let config: Config = generate_config_from_arguments(endpoints, &matches);

    /*
    ** Connection to TiVK
    */

    // Create a new connection
    let client = RawClient::new(config).unwrap();

    // Setup client options
    let client_opts: ClientOptions = create_client_opts_from_arguments(&matches);

    println!("{:?}", client_opts);

    let mode: &str = &*client_opts.mode;

    match mode {
        "raw" => {
            println!("Executing command in raw mode");
        },
        "transaction" => {
            println!("Executing command in transaction mode");

            let mut editor = Editor::<()>::new();

            /*if editor.load_history("history.txt").is_err() {
                println!("No previous history to load.");
            }*/

            loop {
                let readline = editor.readline("> ");
                match readline {
                    Ok(line) => {
                        // editor.add_history_entry(line.as_str());

                        // During transaction prompt should be >> instead of >
                        match line.as_str() {
                            "begin" => {
                                // Start transaction
                                println!("Start transaction");
                            },
                            "commit" => {
                                // End transaction
                                println!("End transaction");
                            },
                            _ => {
                                // Commands
                                println!("Line: {}", line);
                            }
                        }

                        // Here should print tikv_client::transaction::TxnInfo
                        // Here should print tikv_client::transaction::Timestamp
                    },
                    Err(ReadlineError::Interrupted) => {
                        println!("CTRL-C");
                        break
                    },
                    Err(ReadlineError::Eof) => {
                        println!("CTRL-D");
                        break
                    },
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break
                    }
                }
            }

            // editor.save_history("history.txt").unwrap();
        }
        _ => {}
    }
}