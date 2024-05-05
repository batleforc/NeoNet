use std::{collections::HashMap, env, fs::read_to_string, path::PathBuf};

use dotenvy::dotenv;
use serde::{Deserialize, Serialize};

use crate::peers::peers_kind::Peer;

const PORT: &str = "PORT";
const HOST_NAME: &str = "HOST_NAME";
const RUST_ENV: &str = "RUST_ENV";

const PERSISTENCE_HOST: &str = "PERSISTENCE_HOST";
const PERSISTENCE_PORT: &str = "PERSISTENCE_PORT";
const PERSISTENCE_USER: &str = "PERSISTENCE_USER";
const PERSISTENCE_PWD: &str = "PERSISTENCE_PWD";
const PERSISTENCE_DB: &str = "PERSISTENCE_DB";
const PERSISTENCE_TLS: &str = "PERSISTENCE_TLS";
const PERSISTENCE_TLS_INSECURE: &str = "PERSISTENCE_TLS_INSECURE";

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub port: u16,
    pub hostname: String,
    pub env: String,
    pub peers: Vec<Peer>,
    pub tracing_sub: Vec<Peer>,
    pub persistence: PersistenceConfig,
    pub auth: Vec<AuthConfig>,
}

impl Config {
    pub fn get_app_name(&self) -> String {
        self.hostname
            .clone()
            .replace("https://", "")
            .replace("http://", "")
            .split(":")
            .collect::<Vec<&str>>()[0]
            .to_string()
    }
}

// Persistence configuration, it will be used to connect to the database
// In the case of a MongoDB database, just add the database and host fields
// ATM, only MongoDB is supported
// In the future, we can add more databases
#[derive(Deserialize, Serialize, Clone)]
pub struct PersistenceConfig {
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
    pub database: String,
    pub tls: Option<bool>,
    pub tls_insecure: Option<bool>,
}

// Auth configuration, it will be used to connect to the authentication service
// It can be build-in or external
// In the case of a build-in authentication service, the kind will be build-in
// The auth configuration should only be used in the file not the environment variables due to it's complexity
#[derive(Deserialize, Serialize, Clone)]
pub struct AuthConfig {
    pub kind: String,
    pub enabled: bool,
    pub require_zkp: bool,
    pub name: String,
    pub version: String,
    pub description: String,
    pub extra_fields: HashMap<String, String>,
}

pub fn parse_local_config(config_file_name: String) -> Config {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    println!(env!("CARGO_MANIFEST_DIR"));
    d.push(config_file_name.clone());
    match dotenv() {
        Ok(_) => println!("Loaded .env file"),
        Err(err) => println!("No .env file found: {:?}", err),
    }
    parse_config(d, config_file_name)
}

pub fn parse_test_config() -> Config {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_config.yaml");
    parse_config(d, "test_config.yaml".to_string())
}

pub fn parse_config(path_buf: PathBuf, config_file_name: String) -> Config {
    let config: Config = parse_config_from_file(path_buf, config_file_name);
    override_config_with_env_vars(config)
}

fn parse_config_from_file(path_buf: PathBuf, config_file_name: String) -> Config {
    let config_file = path_buf.into_os_string().into_string().unwrap();
    let binding = read_to_string(config_file).unwrap();
    let config_content = binding.as_str();
    match config_file_name.split('.').last() {
        Some("yaml" | "yml") => serde_yaml::from_str(config_content).unwrap(),
        Some("toml") => toml::from_str(config_content).unwrap(),
        Some("json") => serde_json::from_str(config_content).unwrap(),
        Some(&_) => {
            panic!("No valid config file provided {}", config_file_name)
        }
        None => {
            panic!("No valid config file provided {}", config_file_name)
        }
    }
}

fn override_config_with_env_vars(config: Config) -> Config {
    let pers = config.persistence;
    Config {
        port: env::var(PORT)
            .unwrap_or(config.port.to_string())
            .parse::<u16>()
            .unwrap(),
        hostname: env::var(HOST_NAME).unwrap_or(config.hostname),
        env: env::var(RUST_ENV).unwrap_or(config.env),
        peers: config.peers,
        tracing_sub: config.tracing_sub,
        persistence: PersistenceConfig {
            host: env::var(PERSISTENCE_HOST).unwrap_or(pers.host),
            port: env::var(PERSISTENCE_PORT)
                .map(|p| {
                    p.parse::<u16>()
                        .expect("Cannot parse the received persistence port")
                })
                .ok()
                .or(pers.port),
            user: env::var(PERSISTENCE_USER).unwrap_or(pers.user),
            password: env::var(PERSISTENCE_PWD).unwrap_or(pers.password),
            database: env::var(PERSISTENCE_DB).unwrap_or(pers.database),
            tls: env::var(PERSISTENCE_TLS)
                .map(|p| {
                    p.parse::<bool>()
                        .expect("Cannot parse the received persistence tls")
                })
                .ok()
                .or(pers.tls),
            tls_insecure: env::var(PERSISTENCE_TLS_INSECURE)
                .map(|p| {
                    p.parse::<bool>()
                        .expect("Cannot parse the received persistence tls")
                })
                .ok()
                .or(pers.tls_insecure),
        },
        auth: config.auth,
    }
}
