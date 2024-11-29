pub const ENV_TEMPLATE: &str = r#"KINTONE_BASE_URL=https://example.kintone.com
KINTONE_USERNAME=your-username
KINTONE_PASSWORD=your-password
KINTONE_APP_ID=your-app-id"#;

pub const GITIGNORE_TEMPLATE: &str = r#".env
node_modules
dist"#;

// src/files/templates.rs の CUSTOMIZE_MANIFEST_TEMPLATE を修正
pub const CUSTOMIZE_MANIFEST_TEMPLATE: &str = r#"{
  "app": "",
  "scope": "ALL",
  "desktop": {
    "js": ["dist/bundle.js"],
    "css": []
  },
  "mobile": {
    "js": [],
    "css": []
  }
}"#;

pub fn package_json_template(project_name: &str) -> String {
    format!(
        r#"{{
  "name": "{}",
  "version": "1.0.0",
  "main": "index.js",
  "scripts": {{
    "start": "node upload.js",
    "build": "webpack --mode production",
    "test": "echo \"Error: no test specified\" && exit 1"
  }},
  "devDependencies": {{
    "@kintone/customize-uploader": "^8.0.13",
    "dotenv": "^16.4.5",
    "ts-loader": "^9.5.1",
    "typescript": "^5.6.3",
    "webpack": "^5.96.1",
    "webpack-cli": "^5.1.4"
  }}
}}"#,
        project_name
    )
}

pub const TSCONFIG_TEMPLATE: &str = r#"{
  "compilerOptions": {
    "target": "ES6",
    "module": "ESNext",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "typeRoots": ["./node_modules/@types", "./src/types"]
  },
  "include": ["src/**/*"]
}"#;

pub const UPLOAD_JS_TEMPLATE: &str = r#"require("dotenv").config();
const fs = require("fs");
const { spawn } = require("child_process");

// カスタマイズマニフェストを動的に生成
const manifestJson = {
  app: process.env.KINTONE_APP_ID,
  scope: "ALL",
  desktop: {
    js: ["dist/bundle.js"],
    css: []
  },
  mobile: {
    js: [],
    css: []
  }
};

// マニフェストファイルを書き出し
fs.writeFileSync("customize-manifest.json", JSON.stringify(manifestJson, null, 2));

const mode = process.env.NODE_ENV || "development";

let isDeploying = false;
let deploymentCheckCount = 0;
const MAX_DEPLOYMENT_CHECKS = 10; // 最大チェック回数

const webpackProcess = spawn("npx", ["webpack", "--watch", "--mode", mode], {
  shell: true,
});

webpackProcess.stdout.on("data", (data) => {
  const message = data.toString();
  console.log(`Webpack: ${message}`);

  if (message.includes("built")) {
    console.log("Webpack build completed. Starting upload...");

    const uploadProcess = spawn(
      "npx",
      [
        "kintone-customize-uploader",
        "customize-manifest.json",
        "--base-url",
        process.env.KINTONE_BASE_URL,
        "--username",
        process.env.KINTONE_USERNAME,
        "--password",
        process.env.KINTONE_PASSWORD,
      ],
      { shell: true }
    );

    uploadProcess.stdout.on("data", (uploadData) => {
      const uploadMessage = uploadData.toString();
      console.log(`Uploader: ${uploadMessage}`);

      // デプロイ待機状態の検出
      if (uploadMessage.includes("運用環境への反映の完了を待っています")) {
        isDeploying = true;
        deploymentCheckCount++;
      }

      // デプロイ完了の検出
      if (uploadMessage.includes("運用環境に反映しました")) {
        console.log("✨ デプロイが完了しました！");
        process.exit(0); // 正常終了
      }

      // デプロイ待機が長すぎる場合の処理
      if (isDeploying && deploymentCheckCount >= MAX_DEPLOYMENT_CHECKS) {
        console.log("⚠️ デプロイ待機時間が長すぎるため、プロセスを終了します");
        process.exit(1); // エラー終了
      }
    });

    uploadProcess.stderr.on("data", (uploadError) => {
      console.error(`Uploader Error: ${uploadError}`);
      process.exit(1); // エラー終了
    });

    uploadProcess.on("error", (error) => {
      console.error(`Uploader Process Error: ${error}`);
      process.exit(1); // エラー終了
    });
  }
});

webpackProcess.stderr.on("data", (data) => {
  console.error(`Webpack Error: ${data}`);
});

webpackProcess.on("error", (error) => {
  console.error(`Webpack Process Error: ${error}`);
  process.exit(1); // エラー終了
});

// プロセス終了時の処理
process.on("SIGINT", () => {
  console.log("プロセスを終了します...");
  process.exit(0);
});
"#;

pub const WEBPACK_CONFIG_TEMPLATE: &str = r#"const path = require("path");

module.exports = {
  entry: "./src/index.ts",
  output: {
    filename: "bundle.js",
    path: path.resolve(__dirname, "dist"),
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
};"#;

pub const INDEX_TS_TEMPLATE: &str = r#"const events = ["app.record.create.submit"];

kintone.events.on(events, (event) => {
  alert("レコードが追加されました");
  return event;
});"#;

pub const KINTONE_TYPES_TEMPLATE: &str = include_str!("../../templates/kintone.d.ts");
