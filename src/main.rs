use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_code = &args[1];
    let log_levels: Vec<String> = args[2].split(",").map(|log| log.to_string()).collect();
    
    let pattern = format!(r"\s*console\.({})\s*\(.*?\);?",log_levels.join("|"));
    
    let regex = Regex::new(&pattern).unwrap();
    
    let removed_console_code = regex.replace_all(&source_code, "");
    
    let removed_unicode_code = removed_console_code.replace(r"\n", "").replace(r"\t", "").replace(r"\r", "").replace(r"\s", "");

    let input = r#"import { jsx } from \"react/jsx-runtime\";\nimport { StrictMode } from \"react\";\n\timport { createRoot } from \"react-dom/client\";\r\n"#;
    println!("{}", input.escape_unicode().to_string());

    println!("{}", removed_unicode_code);
}
