use wasm_bindgen::prelude::*;
use regex::Regex;

#[wasm_bindgen]
pub fn remove_console_from_levels(str: &str, log_levels: Vec<String>) -> String {
    
    let pattern = format!(r"\s*console\.({})\s*\(.*?\);?",log_levels.join("|"));
    
    let regex = Regex::new(&pattern).unwrap();

    let removed_console_code = regex.replace_all(str, "");
    
    removed_console_code.replace(r"\n", "").replace(r"\t", "").replace(r"\r", "").replace(r"\s", "").replace(r"\v", "").replace(r"\b", "").to_string()
}