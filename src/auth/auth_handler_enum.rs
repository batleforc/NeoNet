use std::fmt::Display;

#[derive(Debug)]
pub enum LoginRequestRetun {
    JWT(String),
    REDIRECT(String),
}

#[derive(Debug)]
pub enum LoginRequestError {
    InvalidData(String),
    Unknown(String),
}

impl Display for LoginRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginRequestError::InvalidData(msg) => write!(f, "InvalidData: {}", msg),
            LoginRequestError::Unknown(msg) => write!(f, "Unknown: {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum CallbackRequestError {
    InvalidData(String),
    Unknown(String),
}

impl Display for CallbackRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallbackRequestError::InvalidData(msg) => write!(f, "InvalidData: {}", msg),
            CallbackRequestError::Unknown(msg) => write!(f, "Unknown: {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum LogoutRequestError {
    InvalidData(String),
    Unknown(String),
}

impl Display for LogoutRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogoutRequestError::InvalidData(msg) => write!(f, "InvalidData: {}", msg),
            LogoutRequestError::Unknown(msg) => write!(f, "Unknown: {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum ValidateRequestError {
    InvalidData(String),
    Unknown(String),
}

impl Display for ValidateRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidateRequestError::InvalidData(msg) => write!(f, "InvalidData: {}", msg),
            ValidateRequestError::Unknown(msg) => write!(f, "Unknown: {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum RefreshRequestError {
    InvalidData(String),
    Unknown(String),
}

impl Display for RefreshRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefreshRequestError::InvalidData(msg) => write!(f, "InvalidData: {}", msg),
            RefreshRequestError::Unknown(msg) => write!(f, "Unknown: {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum RegisterRequestError {
    InvalidData(String),
    Unknown(String),
}
