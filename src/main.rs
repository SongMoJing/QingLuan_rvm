mod _lib;
mod run;

use std::collections::LinkedList;
use clap::{Arg, Command};
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
    let software = args.next().unwrap();
    let action = args.next().unwrap();
    // 检查文件是否存在
    if !std::path::Path::new(&software).exists() {
        base::system("File not found");
    }
    if !std::fs::File::open(&software).unwrap().metadata().unwrap().permissions().readonly() {
        base::system("File is not readable");
    }
    if action == "run" {
        run::main(software);
    }
    exit(0);
}

/// ## 退出程序
/// 提示用户按任意键继续
fn exit(exit_code: i32) {
    base::system("please");
    std::process::exit(exit_code);
}

/// ## 获取命令行参数
/// 读入必要参数
fn get_args() -> LinkedList<String> {
    let mut res: LinkedList<String> = LinkedList::new();
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        // .arg_required_else_help(true)
        .arg(Arg::new("software")
            .value_name("PATH.qls")
            .help("Specify the Qingluan software package to be run")
            .required(true))
        .arg(Arg::new("active")
            .short('a')
            .long("active")
            .value_name("run|output")
            .help("Specifies how the Qingluan package is running")
            .required(false)
            .default_value("run"))
        .get_matches();

    // 获取参数
    // 路径
    let mut path = matches.get_one::<String>("software").unwrap().to_string();
    path = match path.eq(".") {
        true => std::env::current_dir().unwrap().to_str().unwrap().to_string().replace("\\", "/"),
        _ => path.replace("\\", "/"),
    };
    res.push_back(path.to_string());
    // 操作
    res.push_back(matches.get_one::<String>("active").unwrap().to_string().to_string());
    res
}
