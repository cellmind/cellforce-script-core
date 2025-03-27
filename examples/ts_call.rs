use std::time::Instant;
use cellforce_script_core::runner::builder::ScriptFunctionRunnerBuilder;

fn main() {
    let long_text =
        "For convenience, when a BLOB is appended to a string, or vice versa, it is treated as a UTF-8 encoded byte stream and automatically first converted into the appropriate string value. That is because it is <src-text> rarely useful to append a BLOB into a string, but extremely useful to be able to directly manipulate UTF-8 encoded text.";

        let script = r#"
         function replace(a: string): string {
        return a.replace("<src-text>", "<target-text>");
    }
    "#;
    let runner = ScriptFunctionRunnerBuilder::new()
        .build("ts", script, "replace")
        .unwrap();

    let timer = Instant::now();

    for _ in 0..1000000 {
        let result = runner.map_in_str_out_str(long_text).unwrap();
    }

    println!("replace: {:?}", timer.elapsed());


    let timer = Instant::now();
    for _ in 0..1000000 {
        let result = long_text.replace("<src-text>", "<target-text>");
    }
    println!("replace: {:?}", timer.elapsed());

}