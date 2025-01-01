

use std::path::PathBuf;
use std::sync::Arc;
use cellgen_script_core::runner::builder::ScriptFunctionRunnerBuilder;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_javascript() {

    let script = r#"
    function add_suffix(a) {
        return a + "-suffix";
    }
    "#;

    let runner = ScriptFunctionRunnerBuilder::new()
        .build("js", script, "add_suffix")
        .unwrap();

    let result = runner.map_in_str_out_str("hello").unwrap();

    assert_eq!(result, "hello-suffix");


}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_typescript() {

    let script = r#"
    function add_suffix(a: string): string {
        return a + "-suffix";
    }
    "#;

    let runner = ScriptFunctionRunnerBuilder::new()
        .build("ts", script, "add_suffix")
        .unwrap();

    let result = runner.map_in_str_out_str("hello").unwrap();

    assert_eq!(result, "hello-suffix");


}
