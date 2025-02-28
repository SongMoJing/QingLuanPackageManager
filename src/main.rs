use std::collections::HashMap;
use std::process::exit;

use colored::*;

use crate::_lib::io::{Log, LogType};

mod _lib;

/// ## 程序版本
const VERSION: &str = "t.0.1";
/// ## 程序名称
const NAME: &str = "\"青鸾\" 包管理器";
/// ## 程序作者
const AUTHOR: &str = "PRC.松蓦箐 <Song_Mojing@outlook.com>";

/// ## 启用 ANSI 支持
fn enable_ansi_support() {
    let _ = control::set_virtual_terminal(true);
}

fn main() {
    #[cfg(windows)]
    enable_ansi_support();
    // 获得参数
    let args = get_args();
    // 检查开始方式
    if let Some(root_path) = args.get("projectPath") {
        // 查看projectPath是否存在
        if std::fs::metadata(args.get("projectPath").unwrap()).is_ok() {
            // 读取QingLuan.toml文件
            script::start(root_path.to_string());
        } else {
            Log::new(LogType::Err, format!("路径 {} 不存在。", args.get("projectPath").unwrap()).as_str(), 21).print();
        }
    }
}

/// ## 获取命令行参数
/// 读入操作和必要参数<br>
/// scriptPath. 路径
fn get_args() -> HashMap<String, String> {
    let mut res: HashMap<String, String> = HashMap::new();

    // 打印帮助
    fn help() {
        println!("{}", "帮助".yellow());
        println!("   {} 版本：{}", NAME.green(), VERSION.green());
        println!("   {}", AUTHOR);
        println!("{}", "用法：".yellow());
        println!("   QingLuanPackageManager [选项]");
        println!("   QingLuanPackageManager [子命令] [参数] [选项]");
        println!("{}", "选项：".yellow());
        println!("   <-h | --help>            获取帮助");
        println!("{}", "子命令：".yellow());
        println!("   init           初始化项目");
        println!("	[项目名称] *[-e sdk版本 | 默认: 已安装的最新稳定版本]\n\r");
        println!("   install        安装包");
        println!("   	 [包名称] *[-v 包版本 | 默认: 远程库中最新版本]\n\r");
        println!("   uninstall      卸载包");
        println!("   	 [包名称] -v [包版本]\n\r");
        println!("   find           查找包");
        println!("   	 [包名称] -v [包版本]\n\r");
        exit(0);
    }

    // 获取参数
    let mut args = std::env::args();
    if args.len() < 2 {
        help();
    } else {
        args.next();
    }

    while let Some(arg) = args.next() {
        if arg.starts_with("-") {
            match arg.as_str() {
                // 显示帮助
                "-h" | "--help" => help(),
                // 未知目标
                _ => {
                    Log::new(LogType::Err, format!("未知的选项 {}。", arg).as_str(), 11).print();
                }
            }
        } else {
            match arg.as_str() {
                "init" => {
                    Log::new(LogType::Err, "初始化项目".as_str(), 13).print();
                }
                "install" => {
                    Log::new(LogType::Err, "安装包".as_str(), 13).print();
                }
                "uninstall" => {
                    Log::new(LogType::Err, "卸载包".as_str(), 13).print();
                }
                "find" => {
                    Log::new(LogType::Err, "查找包", 13).print();
                }
                _ => {
                    Log::new(LogType::Err, format!("未知的子命令 {}。", arg).as_str(), 12).print();
                }
            }
        }
    }
    res
}

