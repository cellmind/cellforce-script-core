use std::sync::Arc;

use arrow::array::{Array, ArrayRef, StringArray};
use arrow::datatypes::DataType;

use crate::errors::ScriptError;

pub trait ScriptFunctionRunner {
    fn str2str(&self, value: &str) -> Result<String, ScriptError>;
    fn str2bool(&self, value: &str) -> Result<bool, ScriptError>;
}
