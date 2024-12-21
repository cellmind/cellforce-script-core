use crate::errors::ScriptError;

pub trait ScriptFunctionRunner {
    fn map_in_str_out_str(&self, value: &str) -> Result<String, ScriptError>;
    fn map_in_str_out_bool(&self, value: &str) -> Result<bool, ScriptError>;
}
