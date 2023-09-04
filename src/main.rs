use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::string;
use colored::*;
use tracing::{event, span, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod handler;
use handler::{convert_to_regex, find_pos};
//芝士模块

fn main() {
    
    tracing_subscriber::registry().with(fmt::layer()).init();
    let span = span!(Level::TRACE, "my_span");
    let enter = span.enter();
    let _guard = span.enter();

    let args: Vec<String> = env::args().collect();
    let _args = args.clone();
    
    // for args_i in args.clone(){
    //     println!("{}",args_i);
    // }
    if args.len() < 3{
        eprintln!("使用方式：{} <目标目录> 114514 <要搜索的正则表达式>", args[0]);
        process::exit(1);
    }

    event!(Level::DEBUG, "日志输出");

    let stop_arg = find_pos(args);
    // println!("{} ",stop_arg);
    let path_args = _args[1..stop_arg as usize].to_vec();
    // println!("I got {:?} arguments: {:?}.", path_args.len(), &path_args[..]);
    let mut regex_args = convert_to_regex(_args[stop_arg as usize +1..].to_vec());
    // println!("I got {:?} arguments: {:?}.", regex_args, &regex_args[..]);
    if regex_args.clone()[regex_args.clone().len()-1].to_string() ==  "-v" || regex_args.clone()[regex_args.clone().len()-1].to_string() == "--verbose"{
        regex_args.pop();
        let mut ret: Vec<String> = Vec::new();

        match _find_v(path_args.clone()) {
            
            // match find(&_args[1], &regex) {
                Ok(matches) => {
                    
                    ret = matches;
                }
                Err(error) => {
                    eprintln!("发生错误：{}", error);
                    process::exit(1);
                }
        }

        match _find(path_args, regex_args) {
            // match find(&_args[1], &regex) {
                Ok(matches) => {
                    println!("here");
                    if matches.is_empty() {
                        println!("未找到匹配项。");
                    }else{
                        println!("找到以下匹配项：");
                        for file in ret{
                            let mut _len = matches.len();
                            let mut _i = 0;
                            let mut ist  = false;
                            while _i < _len{
                                if matches[_i] == file{
                                    ist = true;
                                }
                                _i +=1;
                            }
                            if ist {
                                println!("{}", file.green());
                            }else {
                                println!("{}", file.red());
                            }
                            
                        }
                    }
                }
                Err(error) => {
                    eprintln!("发生错误：{}", error);
                    process::exit(1);
                }
            }
            event!(Level::DEBUG, "-v模式的find");
    }else{    // 这里改传数组就就可以了，然后for循环伺候？
        match _find(path_args, regex_args) {
        // match find(&_args[1], &regex) {
            Ok(matches) => {
                if matches.is_empty() {
                    println!("未找到匹配项。");
                }else{
                    println!("找到以下匹配项：");
                    for file in matches{
                        println!("{}", file.green());
                    }
                }
            }
            Err(error) => {
                eprintln!("发生错误：{}", error);
                process::exit(1);
            }
        }
        event!(Level::DEBUG, "正常的find");
    }
}
// 这里函数没有做改动
fn walk_tree(
    dir: &Path,
    regex: &Regex,
    matches:&mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>>{
    if dir.is_dir(){
        for entry in fs::read_dir(dir)?{
            let entry = entry?;
            let path = entry.path();
            if path.is_dir(){
                walk_tree(&path, regex, matches)?;
            }else if let Some(filename) =path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename){
                    matches.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}
// 就是传了一个vec进去
fn _find<P: AsRef<Path>>(root: Vec<P>, regex: Vec<Regex>) ->Result<Vec<String>, Box<dyn std::error::Error>>{
    let mut matches = Vec::new();
    for root_i in root{
            for regex_i in regex.clone() {
                walk_tree(root_i.as_ref(), &regex_i, &mut  matches)?;
            }
    }
    matches.sort();
    let mut len = matches.len();
    let mut i = 0;
    while i+1 < len {
        if matches[i] == matches[i+1] {
            matches.remove(i);
            len = len - 1;
        }else{
            i = i + 1;
        }
    }
    Ok(matches)
}

fn _find_v<P: AsRef<Path>>(root: Vec<P>) ->Result<Vec<String>, Box<dyn std::error::Error>>{
    let mut matches = Vec::new();
    for root_i in root{
                walk_tree_v(root_i.as_ref(), &mut  matches)?;
    }
    matches.sort();
    let mut len = matches.len();
    let mut i = 0;
    while i+1 < len {
        if matches[i] == matches[i+1] {
            matches.remove(i);
            len = len - 1;
        }else{
            i = i + 1;
        }
    }
    Ok(matches)
}

fn walk_tree_v(
    dir: &Path,
    matches:&mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>>{
    if dir.is_dir(){
        for entry in fs::read_dir(dir)?{
            let entry = entry?;
            let path = entry.path();
            if path.is_dir(){
                walk_tree_v(&path, matches)?;
            }else if let Some(filename) =path.file_name().and_then(|s| s.to_str()) {
                    matches.push(path.to_string_lossy().to_string());
            }
        }
    }
    Ok(())
}