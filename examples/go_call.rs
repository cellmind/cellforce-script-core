use std::time::Instant;
use cellgen_script_core::runner::builder::ScriptFunctionRunnerBuilder;

fn main() {

    let long_text =
        "For convenience, when a BLOB is appended to a string, or vice versa, it is treated as a UTF-8 encoded byte stream and automatically first converted into the appropriate string value. That is because it is <src-text> rarely useful to append a BLOB into a string, but extremely useful to be able to directly manipulate UTF-8 encoded text.";



    let script = r#"
    import "strings"
    import "os"
    func replace(s string) string {
        v := "For convenience, when a BLOB is appended to a string, or vice versa, it is treated as a UTF-8 encoded byte stream and automatically first converted into the appropriate string value. That is because it is <src-text> rarely useful to append a BLOB into a string, but extremely useful to be able to directly manipulate UTF-8 encoded text."
        v2 := strings.ReplaceAll(v, "<src-text>", "<target-text>")
        return v2
    }
    "#;


    let timer = Instant::now();

    let runner = ScriptFunctionRunnerBuilder::new()
        .build("go", script, "replace")
        .unwrap();


    println!("replace: {:?}", timer.elapsed());

    let timer = Instant::now();

    for _ in 0..10000000 {
        let result = runner.map_in_str_out_str(long_text).unwrap();
    }

    println!("replace: {:?}", timer.elapsed());


    let timer = Instant::now();
    for _ in 0..10000000 {
        let result = long_text.replace("<src-text>", "<target-text>");
    }
    println!("replace: {:?}", timer.elapsed());

}