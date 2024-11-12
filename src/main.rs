mod _lib;

use std::collections::LinkedList;
use clap::{Arg, ArgMatches, Command};
use colored::*;
use crate::_lib::base;

const VERSION: &str = "S.0.1:2024";
const NAME: &str = "\"青鸾\" Universal Runtime Virtual Machine.";
const AUTHOR: &str = "PRC.松蓦箐 <Song_Mojing@outlook.com>";

/// ## 启用 ANSI 支持
fn enable_ansi_support() {
    let _ = control::set_virtual_terminal(true);
}

fn main() {
    #[cfg(windows)]
    enable_ansi_support();

    let mut args = get_args().into_iter();

    println!("path:    {}", args.next().unwrap());
    println!("action:  {}", args.next().unwrap());
    println!("version: {}", args.next().unwrap());

    exit(0)
}

/// ## 退出程序
/// 提示用户按任意键继续
fn exit(exit_code: i32) {
    base::system("please");
    std::process::exit(exit_code);
}

/// ## 获取命令行参数
/// 读入操作和必要参数
/// 0. 路径
/// 1. 操作
/// 2. 参数...
fn get_args() -> LinkedList<String> {
    let mut res: LinkedList<String> = LinkedList::new();
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        // .arg_required_else_help(true)
        .arg(Arg::new("path")
            .short('p')
            .long("path")
            .value_name("PATH")
            .help("Project path")
            .required(false)
            .default_value("."))
        .arg(Arg::new("active")
            .short('a')
            .long("active")
            .value_name("run|output")
            .help("Specifies how the Qingluan package is running")
            .required(false)
            .default_value("run"))
        .arg(Arg::new("rVersion")
            .short('v')
            .long("rVersion")
            .value_name("VERSION")
            .help("Specifies the running version of the 青鸾 package")
            .required(false)
            .default_value(VERSION))
        .get_matches();

    // 获取参数
    // 路径
    let mut path = matches.get_one::<String>("path").unwrap().to_string();
    path = match path.eq(".") {
        true => std::env::current_dir().unwrap().to_str().unwrap().to_string().replace("\\", "/"),
        _ => path.replace("\\", "/"),
    };
    res.push_back(path.to_string());
    // 操作
    res.push_back(matches.get_one::<String>("active").unwrap().to_string().to_string());
    // 参数
    res.push_back(matches.get_one::<String>("rVersion").unwrap().to_string().to_string());
    res
}
