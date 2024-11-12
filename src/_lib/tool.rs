use std::env;
use std::fs::{File};
use std::path::Path;
use xmltree::Element;

/// 获取青鸾HOME路径
fn get_path() -> String {
    let qingluan_home = env::var("QINGLUAN_HOME");
    if qingluan_home.is_err() {
        let user_home = env::var("HOME");
        if user_home.is_err() {
            "null".to_string()
        } else {
            format!("{}/.qingluan", user_home.unwrap())
        }
    } else {
        qingluan_home.unwrap()
    }
}

/// 获取SDK配置
pub(crate) fn get_sdk(token: String) -> String {
    let file_path = format!("{}/conf/sdk.xml", get_path().trim_matches('"'));
    if !Path::new(&file_path).exists() {
        "null: 没有配置文件，尝试重新安装青鸾SDK".to_string()
    } else {
        let root = Element::parse(File::open(file_path).unwrap()).unwrap();
        if let Some(sdk) = root.get_child(&*token) {
            if let Some(value) = sdk.get_text() {
                value.to_string()
            } else {
                format!("null: 没有{}配置内容，尝试重新安装青鸾SDK", &token).to_string()
            }
        } else {
            "null: 没有SDK配置内容，尝试重新安装青鸾SDK".to_string()
        }
    }
}