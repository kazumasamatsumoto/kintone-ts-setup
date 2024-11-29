use crate::files::templates;
use std::fs;
use std::process::Command;

pub fn create_directory_structure() -> std::io::Result<()> {
    println!("📁 ディレクトリ構造を作成中...");
    fs::create_dir_all("src")?;
    fs::create_dir_all("src/types")?;
    fs::create_dir_all("dist")?;
    Ok(())
}

pub fn create_env_file() -> std::io::Result<()> {
    fs::write(".env", templates::ENV_TEMPLATE)?;
    Ok(())
}

pub fn create_gitignore() -> std::io::Result<()> {
    fs::write(".gitignore", templates::GITIGNORE_TEMPLATE)?;
    Ok(())
}

pub fn create_customize_manifest() -> std::io::Result<()> {
    fs::write(
        "customize-manifest.json",
        templates::CUSTOMIZE_MANIFEST_TEMPLATE,
    )?;
    Ok(())
}

pub fn create_package_json(project_name: &str) -> std::io::Result<()> {
    let content = templates::package_json_template(project_name);
    fs::write("package.json", content)?;
    Ok(())
}

pub fn create_tsconfig() -> std::io::Result<()> {
    fs::write("tsconfig.json", templates::TSCONFIG_TEMPLATE)?;
    Ok(())
}

pub fn create_upload_js() -> std::io::Result<()> {
    fs::write("upload.js", templates::UPLOAD_JS_TEMPLATE)?;
    Ok(())
}

pub fn create_webpack_config() -> std::io::Result<()> {
    fs::write("webpack.config.js", templates::WEBPACK_CONFIG_TEMPLATE)?;
    Ok(())
}

pub fn create_index_ts() -> std::io::Result<()> {
    fs::write("src/index.ts", templates::INDEX_TS_TEMPLATE)?;
    Ok(())
}

pub fn create_kintone_types() -> std::io::Result<()> {
    fs::write("src/types/kintone.d.ts", templates::KINTONE_TYPES_TEMPLATE)?;
    Ok(())
}

pub fn initialize_npm_project() -> std::io::Result<()> {
    println!("📦 npmパッケージをインストール中...");
    Command::new("npm").arg("install").status()?;
    Ok(())
}
