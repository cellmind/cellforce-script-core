
use std::panic;
use std::any::Any;
use crate::runner::base::ScriptFunctionRunner;
use crate::errors::ScriptError;

use crate::runner::go::interpreter::{FreeGoScriptInterpreterRequest, GoScriptCall, GoScriptCallImpl, MapInStrOutStrRequest, NewGoScriptInterpreterRequest};


fn box_to_string(boxed: Box<dyn Any + Send>) -> Option<String> {
    if let Some(string) = boxed.downcast_ref::<String>() {
        return Some(string.clone());
    }
    Some(format!("{:?}", boxed))
}

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

        let resp = panic::catch_unwind(|| {
            GoScriptCallImpl::new_interpreter(&req)
        }).map_err(|e| {
            ScriptError::GeneralError {
                msg: format!("failed to create interpreter: {:?}", box_to_string(e).unwrap_or("failed to create interpreter".to_string())),
            }
        })?;
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
        let result = panic::catch_unwind(|| {
            GoScriptCallImpl::free_interpreter(&req)
        });
        match result {
            Ok(resp) => {
                if resp.error != "" {
                    eprintln!("failed to free interpreter: {:?}", resp.error);
                }
            }
            Err(e) => {
                eprintln!("failed to free interpreter: {:?}", box_to_string(e).unwrap_or("failed to free interpreter".to_string()));
            }
        }
    }
}

impl ScriptFunctionRunner for GolangFunctionRunner {
    fn map_in_str_out_str(&self, value: &str) -> Result<String, ScriptError> {
        let req = MapInStrOutStrRequest {
            ptr_offset: self.ptr_offset,
            value: value.to_string()
        };
        let resp = panic::catch_unwind(|| {
            GoScriptCallImpl::map_in_str_out_str(&req)
        }).map_err(|e| {
            ScriptError::GeneralError {
                msg: format!("failed to call function: {:?}", box_to_string(e).unwrap_or("failed to call function".to_string())),
            }
        })?;
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
