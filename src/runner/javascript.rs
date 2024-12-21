use std::cell::RefCell;
use std::sync::Arc;

use boa_engine::{Context, Source};
use crate::runner::base::ScriptFunctionRunner;
use crate::errors::ScriptError;

pub struct JavaScriptFunctionRunner {
    js_context: Arc<RefCell<Context>>,
    script: String,
    func: String,
}

impl JavaScriptFunctionRunner {
    pub fn try_new(
        script: &str,
        func: &str,
    ) -> Result<Self, ScriptError> {
        let mut js_context = Context::default();
        js_context.runtime_limits_mut().set_loop_iteration_limit(10);
        js_context
            .eval(Source::from_bytes(script))
            .map_err(|e| e.to_string())?;
        Ok(Self {
            script: script.to_string(),
            js_context: Arc::new(RefCell::new(js_context)),
            func: func.to_string(),
        })
    }
}

impl ScriptFunctionRunner for JavaScriptFunctionRunner {
    fn map_in_str_out_str(&self, value: &str) -> Result<String, ScriptError> {
        let call_script = r#"
        {self.func}("{value}")
        "#;
        let result = self
            .js_context
            .borrow_mut()
            .eval(Source::from_bytes(call_script))
            .map_err(|e| e.to_string())?;
        match result.as_string() {
            Some(v) => Ok(v.to_std_string().map_err(|e| e.to_string())?),
            None => Err(ScriptError::GeneralError {
                msg: format!("call {} {} got error", self.func, value),
            }),
        }
    }

    fn map_in_str_out_bool(&self, value: &str) -> Result<bool, ScriptError> {
        let call_script = r#"
        {self.func}("{value}")
        "#;
        let result = self
            .js_context
            .borrow_mut()
            .eval(Source::from_bytes(call_script))
            .map_err(|e| e.to_string())?;
        match result.as_boolean() {
            Some(v) => Ok(v),
            None => Err(ScriptError::GeneralError {
                msg: format!("call {} {} got error", self.func, value),
            }),
        }
    }
}
