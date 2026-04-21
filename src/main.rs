use axum::Router;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tower_http::services::{ServeDir, ServeFile};

use crate::base::create_base_router;

pub mod base;
// 启动函数
#[tokio::main]
async fn main() {
    init_config();
    let config = CFG.get().unwrap();
    let base = create_base_router();
    let app = Router::new().nest("/api", base).fallback_service(
        ServeDir::new("static")
            .append_index_html_on_directories(true)
            .not_found_service(ServeFile::new("static/index.html")),
    );
    let listener = tokio::net::TcpListener::bind(&config.host).await.unwrap();
    for (idx, key) in config.keys.clone().iter().enumerate() {
        println!("可用的apikey-{}: {}", idx, key)
    }
    println!("服务运行于: {}", config.host);
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize, Default, Serialize, Clone)]
struct Config {
    host: String,
    keys: Vec<String>,
    pub base: Option<Base>,
}
#[derive(Deserialize, Serialize, Clone, Default)]
struct Base {
    web_title: Option<String>,
    company: Option<String>,
}

// 全局静态配置
static CFG: OnceLock<Config> = OnceLock::new();

pub fn init_config() {
    let cfg_str = std::fs::read_to_string("config.json").expect("无法读取 config.json");
    let cfg: Config = serde_json::from_str(&cfg_str).expect("解析 config.json 失败");
    CFG.set(cfg).ok();
}
