use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::string;
pub fn convert_to_regex(a:Vec<String>)->Vec<Regex>{
    let mut regex_box:Vec<Regex> = Vec::new();
    for i in a{
        let pattern = &i;
        match Regex::new(pattern){
            Ok(re) => regex_box.push(re),
            Err(err) =>{
                eprintln!("无效的正则表达式'{}':{}", pattern,err);
                process::exit(1);
            }
        };
    }
    regex_box
}