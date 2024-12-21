use std::sync::Arc;
//
// use indexmap;
// use rustpython::vm;
// use rustpython_vm::{Interpreter, PyObjectRef, Settings};
// use rustpython_vm::function::{FuncArgs, KwArgs};
//
// use crate::errors::QueryError;
// use crate::query::engine::FunctionContext;
// use crate::query::function::buildin::script::ScriptFunctionRunner;
//
// pub struct PythonFunctionRunner {
//     context: Arc<FunctionContext>,
//     interp: Interpreter,
//     script: String,
//     call_fn: PyObjectRef,
// }
//
// impl PythonFunctionRunner {
//     pub fn try_new(
//         context: Arc<FunctionContext>,
//         script: &str,
//         func: &str,
//     ) -> Result<Self, QueryError> {
//         let mut settings = Settings::default();
//         settings.debug = true;
//         settings.quiet = false;
//         settings.verbose = 10;
//
//         let interp = rustpython::InterpreterConfig::new()
//             .init_stdlib()
//             .init_hook(Box::new(|vm| {}))
//             .settings(settings)
//             .interpreter();
//
//         let call_fn = interp.enter(|vm| {
//             let scope = vm.new_scope_with_builtins();
//             let code_obj = vm
//                 .compile(script, vm::compiler::Mode::Exec, "<embedded>".to_owned())
//                 .map_err(|err| vm.new_syntax_error(&err, Some(script)))
//                 .unwrap();
//
//             vm.run_code_obj(code_obj, scope.clone()).unwrap();
//             let call_fn = scope.globals.get_item(func, vm).unwrap();
//             call_fn
//         });
//
//         Ok(Self {
//             context,
//             script: script.to_string(),
//             interp,
//             call_fn,
//         })
//     }
// }
//
// impl ScriptFunctionRunner for PythonFunctionRunner {
//     fn str2str(&self, value: &str) -> Result<String, QueryError> {
//         self.interp.enter(|vm| {
//             let args = vec![vm.ctx.new_str("world").into()];
//             let mut kwargs: indexmap::IndexMap<String, PyObjectRef> = indexmap::IndexMap::new();
//             let mut kwargs = KwArgs::new(kwargs);
//             let args = FuncArgs::new(args, kwargs);
//             let result = self.call_fn.call_with_args(args, vm).unwrap();
//             let result = result
//                 .str(vm)
//                 .map_err(|e| format!("failed to call function: {:?}", e))?;
//             Ok(result.to_string())
//         })
//     }
//
//     fn str2bool(&self, value: &str) -> Result<bool, QueryError> {
//         self.interp.enter(|vm| {
//             let args = vec![vm.ctx.new_str("world").into()];
//             let mut kwargs: indexmap::IndexMap<String, PyObjectRef> = indexmap::IndexMap::new();
//             let mut kwargs = KwArgs::new(kwargs);
//             let args = FuncArgs::new(args, kwargs);
//             let result = self.call_fn.call_with_args(args, vm).unwrap();
//             let result = result
//                 .try_to_bool(vm)
//                 .map_err(|e| format!("failed to call function: {:?}", e))?;
//             Ok(result)
//         })
//     }
// }
