use criterion::{criterion_group, criterion_main, Criterion};
use cellgen_script_core::runner::builder::ScriptFunctionRunnerBuilder;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("scripts");

    group.bench_function("run rhai add suffix", |b| {
        let script = r#"
    fn add_suffix(a) {
        return a + "-suffix";
    }
    "#;
        let runner = ScriptFunctionRunnerBuilder::new()
            .build("rhai", script, "add_suffix")
            .unwrap();


        b.iter(|| {
            let result = runner.map_in_str_out_str("hello").unwrap();
        })
    });

    group.bench_function("run javascript add suffix", |b| {
        let script = r#"
    function add_suffix(a) {
        return a + "-suffix";
    }
    "#;
        let runner = ScriptFunctionRunnerBuilder::new()
            .build("js", script, "add_suffix")
            .unwrap();

        b.iter(|| {
            let result = runner.map_in_str_out_str("hello").unwrap();
        })
    });

    group.bench_function("run typescript add suffix ", |b| {
        let script = r#"
    function add_suffix(a: string): string {
        return a + "-suffix";
    }
    "#;
        let runner = ScriptFunctionRunnerBuilder::new()
            .build("ts", script, "add_suffix")
            .unwrap();

        b.iter(|| {
            let result = runner.map_in_str_out_str("hello").unwrap();
        })
    });

    let long_text =
    "For convenience, when a BLOB is appended to a string, or vice versa, it is treated as a UTF-8 encoded byte stream and automatically first converted into the appropriate string value. That is because it is <src-text> rarely useful to append a BLOB into a string, but extremely useful to be able to directly manipulate UTF-8 encoded text.";

    group.bench_function("run rhai replace", |b| {
        let script = r#"
    fn replace(a) {
        a.replace("<src-text>", "<target-text>");
        return a;
    }
    "#;
        let runner = ScriptFunctionRunnerBuilder::new()
            .build("rhai", script, "replace")
            .unwrap();


        b.iter(|| {
            let result = runner.map_in_str_out_str(long_text).unwrap();
        })
    });

    group.bench_function("run javascript replace", |b| {
        let script = r#"
    function replace(a) {
        return a.replace("<src-text>", "<target-text>");
    }
    "#;
        let runner = ScriptFunctionRunnerBuilder::new()
            .build("js", script, "replace")
            .unwrap();

        b.iter(|| {
            let result = runner.map_in_str_out_str(long_text).unwrap();
        })
    });

    group.bench_function("run typescript replace", |b| {
        let script = r#"
    function replace(a: string): string {
        return a.replace("<src-text>", "<target-text>");
    }
    "#;
        let runner = ScriptFunctionRunnerBuilder::new()
            .build("ts", script, "replace")
            .unwrap();

        b.iter(|| {
            let result = runner.map_in_str_out_str(long_text).unwrap();
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
