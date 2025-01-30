use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_code = &args[1];
    
    let log_levels: Vec<String> = args.get(2).map(|logs| logs.split(",").map(|log| log.to_string()).collect()).unwrap_or_else(|| vec!["log".to_string()]);
    
    let pattern = format!(r"\s*console\.({})\s*\(.*?\);?",log_levels.join("|"));
    
    let regex = Regex::new(&pattern).unwrap();

    let removed_console_code = regex.replace_all(&source_code, "");
    
    let removed_unicode_code = removed_console_code.replace(r"\n", "").replace(r"\t", "").replace(r"\r", "").replace(r"\s", "").replace(r"\v", "").replace(r"\b", "");

    println!("{}", removed_unicode_code);
}
