use std::fmt::Display;
use envy::Error;

#[derive(Debug, Clone)]
pub enum ConfigException<'a> {
    LoadingEnv(&'a str),
    Unknown(&'a str),
}

impl ConfigException<'_> {
    pub fn message(&self) -> &str {
        match self {
            ConfigException::LoadingEnv(msg) => msg,
            ConfigException::Unknown(msg) => msg,
        }
    }
}

impl Display for ConfigException<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigException::LoadingEnv(msg) => write!(f, "ConfigException (LoadingEnvError): {}", msg),
            ConfigException::Unknown(msg) => write!(f, "ConfigException (UnknownError): {}", msg),
        }
    }
}

impl Into<ConfigException<'_>> for envy::Error {
    fn into(self) -> ConfigException<'static> {
        match self {
            Error::MissingValue(e) => ConfigException::LoadingEnv(e),
            Error::Custom(_) => ConfigException::LoadingEnv("Custom error"),
        }
    }
}