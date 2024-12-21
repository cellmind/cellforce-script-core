use std::sync::Arc;
use crate::runner::base::ScriptFunctionRunner;
use crate::errors::ScriptError;
use crate::runner::javascript::JavaScriptFunctionRunner;
use crate::runner::rhai::RhaiFunctionRunner;

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
        let runner = match lang.to_lowercase().as_str() {
            "rhai" => Arc::new(RhaiFunctionRunner::try_new(
                script,
                func,
            )?) as Arc<dyn ScriptFunctionRunner>,
            // "python" => Arc::new(PythonFunctionRunner::try_new(
            //     script,
            //     func,
            // )?) as Arc<dyn ScriptFunctionRunner>,
            "js" | "javascript" => Arc::new(JavaScriptFunctionRunner::try_new(
                script,
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
}

