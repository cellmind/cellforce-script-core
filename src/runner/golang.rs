
use crate::runner::base::ScriptFunctionRunner;
use crate::errors::ScriptError;

use crate::runner::go::interpreter::{FreeGoScriptInterpreterRequest, GoScriptCall, GoScriptCallImpl, MapInStrOutStrRequest, NewGoScriptInterpreterRequest};

pub struct GolangFunctionRunner {
    ptr_offset: i32,
    function_name: String,
}

impl GolangFunctionRunner {
    pub fn try_new(
        go_script: &str,
        func: &str,
    ) -> Result<Self, ScriptError> {
        let req = NewGoScriptInterpreterRequest {
            function: func.to_string(),
            script: go_script.to_string()
        };
        let resp = GoScriptCallImpl::new_interpreter(&req);
        if resp.error != "" {
            return Err(ScriptError::GeneralError {
                msg: format!("failed to create interpreter: {:?}", resp.error),
            });
        }
        Ok(Self {
            ptr_offset: resp.ptr_offset,
            function_name: func.to_string(),
        })
    }
}

impl Drop for GolangFunctionRunner {
    fn drop(&mut self) {
        let req = FreeGoScriptInterpreterRequest {
            ptr_offset: self.ptr_offset
        };
        let resp = GoScriptCallImpl::free_interpreter(&req);
        if resp.error != "" {
            eprintln!("failed to free interpreter: {:?}", resp.error);
        }
    }
}

impl ScriptFunctionRunner for GolangFunctionRunner {
    fn map_in_str_out_str(&self, value: &str) -> Result<String, ScriptError> {
        let req = MapInStrOutStrRequest {
            ptr_offset: self.ptr_offset,
            value: value.to_string()
        };
        let resp = GoScriptCallImpl::map_in_str_out_str(&req);
        if resp.error != "" {
            return Err(ScriptError::GeneralError {
                msg: format!("failed to call function: {:?}", resp.error),
            });
        }
        Ok(resp.value)
    }

    fn map_in_str_out_bool(&self, value: &str) -> Result<bool, ScriptError> {
        Ok(true)
    }
}
