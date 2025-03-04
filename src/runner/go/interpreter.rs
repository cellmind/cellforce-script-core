pub mod binding {
    #![allow(warnings)]
    rust2go::r2g_include_binding!();
}

#[derive(rust2go::R2G, Clone)]
pub struct NewGoScriptInterpreterRequest {
    pub script: String,
    pub function: String,
}

#[derive(rust2go::R2G, Clone)]
pub struct NewGoScriptInterpreterResponse {
    pub error: String,
    pub ptr_offset: i32,
}

#[derive(rust2go::R2G, Clone)]
pub struct FreeGoScriptInterpreterRequest {
    pub ptr_offset: i32,
}

#[derive(rust2go::R2G, Clone)]
pub struct FreeGoScriptInterpreterResponse {
    pub error: String,
}


#[derive(rust2go::R2G, Clone)]
pub struct MapInStrOutStrRequest {
    pub ptr_offset: i32,
    pub value: String,
}

#[derive(rust2go::R2G, Clone)]
pub struct MapInStrOutStrResponse {
    pub error: String,
    pub value: String
}


#[rust2go::r2g]
pub trait GoScriptCall {
    fn new_interpreter(req: &NewGoScriptInterpreterRequest) -> NewGoScriptInterpreterResponse;
    fn free_interpreter(req: &FreeGoScriptInterpreterRequest) -> FreeGoScriptInterpreterResponse;
    fn map_in_str_out_str(req: &MapInStrOutStrRequest) -> MapInStrOutStrResponse;
}





