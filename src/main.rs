use crate::commands::setup::setup_project;
use std::env;

mod commands;
mod files;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("使用方法: kintone-ts-setup <プロジェクト名>");
        println!("例: kintone-ts-setup my-kintone-project");
        return Ok(());
    }

    let project_name = &args[1];
    setup_project(project_name)
}
