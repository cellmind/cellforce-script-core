use std::sync::Arc;

use rhai::{AST, Engine, Scope};
use crate::runner::base::ScriptFunctionRunner;
use crate::errors::ScriptError;

pub struct RhaiFunctionRunner {
    script: String,
    engine: Engine,
    ast: AST,
    func: String,
}

impl RhaiFunctionRunner {
    pub fn try_new(
        script: &str,
        func: &str,
    ) -> Result<Self, ScriptError> {
        let engine = Engine::new();
        let ast = engine.compile(script).map_err(|e| {
            ScriptError::from(format!("failed to compile script: {:?}", e))
        })?;
        Ok(Self {
            script: script.to_string(),
            engine,
            ast,
            func: func.to_string(),
        })
    }
}

impl ScriptFunctionRunner for RhaiFunctionRunner {
    fn map_in_str_out_str(&self, value: &str) -> Result<String, ScriptError> {
        let mut scope = Scope::new();
        let value = value.to_string();
        let result = self
            .engine
            .call_fn::<String>(
                &mut scope,
                &self.ast,
                self.func.to_string(),
                (value.clone(),),
            )
            .map_err(|e| ScriptError::from(format!("failed to call function: {:?}", e)))?;
        Ok(result)
    }

    fn map_in_str_out_bool(&self, value: &str) -> Result<bool, ScriptError> {
        let mut scope = Scope::new();
        let value = value.to_string();
        let result = self
            .engine
            .call_fn::<bool>(
                &mut scope,
                &self.ast,
                self.func.to_string(),
                (value.clone(),),
            )
            .map_err(|e| ScriptError::from(format!("failed to call function: {:?}", e)))?;
        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_in_str_out_str() {
        let runner = RhaiFunctionRunner::try_new(
            r#"
            fn hello(name) {
                "hello, " + name
            }
            "#,
            "hello",
        ).unwrap();
        let result = runner.map_in_str_out_str("world").unwrap();
        assert_eq!(result, "hello, world");
    }

    #[test]
    fn test_map_in_str_out_bool() {
        let runner = RhaiFunctionRunner::try_new(
            r#"
            fn is_world(name) {
                name == "world"
            }
            "#,
            "is_world",
        ).unwrap();
        let result = runner.map_in_str_out_bool("world").unwrap();
        assert_eq!(result, true);
    }
}