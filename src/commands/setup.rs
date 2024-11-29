use crate::files::creators::*;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn setup_project(project_name: &str) -> std::io::Result<()> {
    let project_dir = PathBuf::from(project_name);

    if project_dir.exists() {
        println!("エラー: {} は既に存在します。", project_name);
        return Ok(());
    }

    println!("🚀 {} プロジェクトの初期化を開始します...", project_name);

    // Create project directory
    fs::create_dir(&project_dir)?;
    env::set_current_dir(&project_dir)?;

    // Create project directory structure
    create_directory_structure()?;

    // Create configuration files
    create_env_file()?;
    create_gitignore()?;
    create_customize_manifest()?;
    create_package_json(project_name)?;
    create_tsconfig()?;
    create_upload_js()?;
    create_webpack_config()?;
    create_index_ts()?;
    create_kintone_types()?;

    // Initialize npm project and install dependencies
    initialize_npm_project()?;

    println!("✨ セットアップが完了しました！");
    // src/commands/setup.rs の setup_project 関数内のメッセージを修正
    println!(
        "
次のステップ:
1. cd {} を実行してプロジェクトディレクトリに移動
2. .envファイルの認証情報を更新:
   - KINTONE_BASE_URL
   - KINTONE_USERNAME
   - KINTONE_PASSWORD
   - KINTONE_APP_ID
3. npm start を実行してビルドと自動アップロードを開始
    ",
        project_name
    );

    Ok(())
}
