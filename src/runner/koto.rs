use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use koto::Koto;
use koto::prelude::*;
use crate::runner::base::ScriptFunctionRunner;
use crate::errors::ScriptError;
use std::borrow::BorrowMut;
use std::ops::Deref;
use koto::bytecode::CompilerSettings;
use indoc::indoc;

pub struct KotoFunctionRunner {
    script: String,
    vm: KotoVm,
    func: String,
    exported_func: KValue,
}

impl KotoFunctionRunner {
    pub fn try_new(
        script: &str,
        func: &str,
    ) -> Result<Self, ScriptError> {
        let settings = KotoVmSettings::default();
        let mut vm = KotoVm::with_settings(settings);

        let chunk = vm.loader().borrow_mut().compile_script(
            script,
            None,
            CompilerSettings {
                export_top_level_ids: true,
                enable_type_checks: true,
            },
        ).map_err(|e| {
            ScriptError::GeneralError {
                msg: format!("failed to compile script: {:?}", e),
            }
        })?;
        vm.run(chunk).map_err(|e| {
            ScriptError::GeneralError {
                msg: format!("failed to run script: {:?}", e),
            }
        })?;


        let exported_func = vm.exports().get(func).ok_or(
            ScriptError::GeneralError {
                msg: format!("function {} not found", func),
            }
        )?;
        if !exported_func.is_callable() {
            return Err(ScriptError::from(format!("function {} is not callable", func)));
        }
        Ok(Self {
            script: script.to_string(),
            func: func.to_string(),
            vm,
            exported_func,
        })
    }
}

impl ScriptFunctionRunner for KotoFunctionRunner {
    fn map_in_str_out_str(&self, value: &str) -> Result<String, ScriptError> {
        let mut vm = self.vm.clone();
        let result = vm.call_function(self.exported_func.clone(), &[value.into()]).map_err(|e| {
            ScriptError::GeneralError {
                msg: format!("failed to call function: {:?}", e),
            }
        })?;
        match result {
            KValue::Str(s) => {
                Ok(s.to_string())
            },
            _ => Err(ScriptError::from(format!("failed to call function: {:?}", result))),
        }
    }

    fn map_in_str_out_bool(&self, value: &str) -> Result<bool, ScriptError> {
        let mut vm = self.vm.clone();
        let result = vm.call_function(self.exported_func.clone(), &[value.into()]).map_err(
            |e| {
                ScriptError::GeneralError {
                    msg: format!("failed to call function: {:?}", e),
                }
            }
        )?;
        match result {
            KValue::Bool(b) => Ok(b),
            _ => Err(ScriptError::from(format!("failed to call function: {:?}", result))),
        }
    }
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_map_in_str_out_str() {
        let runner = KotoFunctionRunner::try_new(
            indoc! {r#"
            export hello = |name|
                'hello, ' + name
            "#},
            "hello"
        ).expect("failed to create KotoFunctionRunner");
        let result = runner.map_in_str_out_str("world").expect("failed to run KotoFunctionRunner");
        assert_eq!(result, "hello, world");
    }

    #[test]
    fn test_map_in_str_out_bool() {
        let runner = KotoFunctionRunner::try_new(
            indoc! {r#"export is_world = |name|
                name == "world"
            "#},
            "is_world",
        ).expect("failed to create KotoFunctionRunner");
        let result = runner.map_in_str_out_bool("world").expect("failed to run KotoFunctionRunner");
        assert_eq!(result, true);
    }
}