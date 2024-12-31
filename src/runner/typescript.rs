use std::cell::RefCell;
use std::sync::Arc;

use boa_engine::{Context, Source};
use crate::runner::base::ScriptFunctionRunner;
use crate::errors::ScriptError;

use swc_common::{
    comments::SingleThreadedComments,
    errors::{ColorConfig, Handler},
    sync::Lrc,
    Globals, Mark, SourceMap, GLOBALS,
};
use swc_ecma_codegen::to_code_default;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};
use swc_ecma_transforms_base::{fixer::fixer, hygiene::hygiene, resolver};
use swc_ecma_transforms_typescript::strip;
use swc_common::BytePos;


pub(crate) fn compile_ts_to_js(ts_script: &str) -> String {

    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let comments = SingleThreadedComments::default();

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: false,
            ..Default::default()
        }),
        Default::default(),
        StringInput::new(&ts_script, BytePos(0), BytePos(ts_script.len() as u32)),
        Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let module = parser
        .parse_program()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("failed to parse module.");

    let globals = Globals::default();
    return GLOBALS.set(&globals, || {
        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();

        // Optionally transforms decorators here before the resolver pass
        // as it might produce runtime declarations.

        // Conduct identifier scope analysis
        let module = module.apply(resolver(unresolved_mark, top_level_mark, true));

        // Remove typescript types
        let module = module.apply(strip(unresolved_mark, top_level_mark));

        // Fix up any identifiers with the same name, but different contexts
        let module = module.apply(hygiene());

        // Ensure that we have enough parenthesis.
        let program = module.apply(fixer(Some(&comments)));

        let js_code = to_code_default(cm, Some(&comments), &program);
        js_code
    });
}

pub struct TypeScriptFunctionRunner {
    js_context: Arc<RefCell<Context>>,
    script: String,
    func: String,
}

impl TypeScriptFunctionRunner {
    pub fn try_new(
        ts_script: &str,
        func: &str,
    ) -> Result<Self, ScriptError> {
        let js_script = compile_ts_to_js(ts_script);

        let mut js_context = Context::default();
        js_context.runtime_limits_mut().set_loop_iteration_limit(10);
        js_context
            .eval(Source::from_bytes(js_script.as_str()))
            .map_err(|e| e.to_string())?;
        Ok(Self {
            script: js_script.to_string(),
            js_context: Arc::new(RefCell::new(js_context)),
            func: func.to_string(),
        })
    }
}

impl ScriptFunctionRunner for TypeScriptFunctionRunner {
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
