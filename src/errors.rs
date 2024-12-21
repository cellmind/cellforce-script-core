


#[derive(thiserror::Error, Debug)]
pub enum ScriptError {
    #[error("general script error: {msg}")]
    GeneralError { msg: String },
}

impl From<String> for ScriptError {
    fn from(msg: String) -> Self {
        ScriptError::GeneralError { msg }
    }
}

impl From<&str> for ScriptError {
    fn from(msg: &str) -> Self {
        ScriptError::GeneralError { msg: msg.into() }
    }
}
