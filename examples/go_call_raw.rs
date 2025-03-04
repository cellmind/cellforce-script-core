use cellgen_script_core::runner::go::interpreter::{FreeGoScriptInterpreterRequest, GoScriptCall, GoScriptCallImpl, MapInStrOutStrRequest, NewGoScriptInterpreterRequest, NewGoScriptInterpreterResponse};

#[tokio::main]
async fn main() {

    let value = "For convenience, when a BLOB is appended to a string, or vice versa, it is treated as a UTF-8 encoded byte stream and automatically first converted into the appropriate string value. That is because it is <src-text> rarely useful to append a BLOB into a string, but extremely useful to be able to directly manipulate UTF-8 encoded text.";

    let src = r#"
    import "strings"
    import "os"
    func replace(s string) string {
        v2 := strings.ReplaceAll(s, "<src-text>", "<target-text>")
        return v2
    }
    "#;


    let req = NewGoScriptInterpreterRequest {
        script: src.to_string(),
        function: "replace".to_string(),
    };
    let resp = GoScriptCallImpl::new_interpreter(&req);
    if resp.error != "" {
        println!("NewGoScriptInterpreterResponse: error: {:?}", resp.error);
        return;
    }
    println!("NewGoScriptInterpreterResponse: error: {:?}", resp.error);


    let map_resp = GoScriptCallImpl::map_in_str_out_str(&MapInStrOutStrRequest {
        ptr_offset: resp.ptr_offset.clone(),
        value: value.to_string(),
    });
    if map_resp.error != "" {
        println!("MapInStrOutStrResponse: error: {:?}", map_resp.error);
        return;
    }
    println!("MapInStrOutStrResponse: {:?}, {:?}", map_resp.error, map_resp.value);


    let resp = GoScriptCallImpl::free_interpreter(&FreeGoScriptInterpreterRequest {
        ptr_offset: resp.ptr_offset,
    });
    if resp.error != "" {
        println!("FreeGoScriptInterpreterResponse: error: {:?}", resp.error);
        return;
    }
    println!("FreeGoScriptInterpreterResponse: error: {:?}", resp.error);
}

