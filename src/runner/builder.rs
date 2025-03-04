use std::sync::Arc;
use crate::runner::base::ScriptFunctionRunner;
use crate::errors::ScriptError;
use crate::runner::golang::GolangFunctionRunner;
use crate::runner::javascript::JavaScriptFunctionRunner;

#[cfg(feature = "python")]
use crate::runner::python::PythonFunctionRunner;

use crate::runner::rhai::RhaiFunctionRunner;
use crate::runner::typescript::TypeScriptFunctionRunner;

pub struct ScriptFunctionRunnerBuilder {
}

impl ScriptFunctionRunnerBuilder {
    pub fn new() -> Self {
        Self { }
    }

    pub fn build(
        &self,
        lang: &str,
        script: &str,
        func: &str,
    ) -> Result<Arc<dyn ScriptFunctionRunner>, ScriptError> {
        let script = Self::preprocess(script)?;
        let runner = match lang.to_lowercase().as_str() {
            "rhai" => Arc::new(RhaiFunctionRunner::try_new(
                script.as_str(),
                func,
            )?) as Arc<dyn ScriptFunctionRunner>,
            #[cfg(feature = "python")]
            "py" | "python" => Arc::new(PythonFunctionRunner::try_new(
                script.as_str(),
                func,
            )?) as Arc<dyn ScriptFunctionRunner>,
            "js" | "javascript" => Arc::new(JavaScriptFunctionRunner::try_new(
                script.as_str(),
                func,
            )?) as Arc<dyn ScriptFunctionRunner>,
            "ts" | "typescript" => Arc::new(TypeScriptFunctionRunner::try_new(
                script.as_str(),
                func,
            )?) as Arc<dyn ScriptFunctionRunner>,
            "go" | "golang" => Arc::new(GolangFunctionRunner::try_new(
                script.as_str(),
                func,
            )?) as Arc<dyn ScriptFunctionRunner>,
            _ => {
                return Err(ScriptError::GeneralError {
                    msg: format!("{lang} script lang not supported"),
                })
            }
        };
        Ok(runner)
    }

    pub fn preprocess(script: &str) -> Result<String, ScriptError> {
        let mut script = script.trim().to_string();
        if script.starts_with("base64:") {
            let decoded_script = base64::decode(&script[7..]).map_err(|e| {
                ScriptError::GeneralError {
                    msg: format!("failed to decode base64 script: {:?}", e),
                }
            })?;
            let decoded_script = String::from_utf8(decoded_script).map_err(|e| {
                ScriptError::GeneralError {
                    msg: format!("failed to convert script to utf8: {:?}", e),
                }
            })?;
            script = decoded_script;
        }
        Ok(script)
    }
}

