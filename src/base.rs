use axum::{
    Json, Router,
    extract::{Path, Request},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_str, from_value, json, to_string};
use tokio::{
    fs::{OpenOptions, create_dir_all, read_dir, read_to_string},
    io::AsyncWriteExt,
};

use crate::{CFG, base::ax_error::AppError};

pub const TDIR: &str = "docs";
pub const BDIR: &str = "backup";

pub fn create_base_router() -> Router {
    // 开放接口：填表端使用
    let public_routes = Router::new()
        .route("/base", get(base))
        .route("/get_tables", get(get_tables))
        .route("/get_table/{:tname}", get(get_table))
        .route("/save_record", post(save_record))
        .route("/check_auth/{:check_auth}", get(check_auth));

    // 管理接口：需要鉴权
    let admin_routes = Router::new()
        .route("/get_records/{:tname}", get(get_records))
        .route("/save_table", post(save_table))
        .route("/del_table/{:tname}", get(del_table))
        .route("/edit_record/{:tname}/{:id}", post(edit_record))
        .route("/del_record/{:tname}/{:id}", post(del_record))
        .layer(middleware::from_fn(admin_auth)); // 应用中间件

    Router::new().merge(public_routes).merge(admin_routes)
}

async fn admin_auth(req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
    // 获取请求头中的 x-api-key
    let auth_header = req.headers().get("x-apikey").and_then(|v| v.to_str().ok());

    let config = CFG.get().expect("配置未初始化");

    if let Some(key) = auth_header {
        if config.keys.contains(&key.to_string()) {
            return Ok(next.run(req).await);
        }
    }

    // 鉴权失败返回 401
    Err(StatusCode::UNAUTHORIZED)
}

async fn base() -> Result<impl IntoResponse, StatusCode> {
    let cfg = CFG.get().cloned().unwrap_or_default();
    return Ok(Json(cfg.base.unwrap_or_default()));
}

async fn get_tables() -> Result<impl IntoResponse, AppError> {
    let bp = std::path::Path::new(TDIR);

    // 如果目录不存在，返回空列表
    if !bp.exists() {
        return Ok(Json(Vec::<QTable>::new()));
    }

    let mut tables = Vec::new();
    let mut entries = read_dir(bp).await.map_err(|e| {
        eprintln!("读取目录失败: {}", e);
        AppError::from(e)
    })?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        // 匹配 .t.json 文件
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.ends_with(".t.json") {
                    // 1. 读取文件内容
                    let content = read_to_string(&path).await.map_err(|e| {
                        eprintln!("读取文件 {:?} 失败: {}", path, e);
                        AppError::from(e)
                    })?;

                    // 2. 反序列化为 QTable 结构体
                    match serde_json::from_str::<QTable>(&content) {
                        Ok(table) => tables.push(table),
                        Err(e) => {
                            // 如果某个文件损坏，记录错误但继续处理其他文件
                            eprintln!("解析文件 {:?} 失败: {}", path, e);
                            continue;
                        }
                    }
                }
            }
        }
    }

    // 返回 QTable 数组，前端会自动收到包含 title, enable 等信息的 JSON 列表
    Ok(Json(tables))
}

async fn get_table(Path(tname): Path<String>) -> Result<impl IntoResponse, AppError> {
    let bp = std::path::Path::new(TDIR);
    let fp = bp.join(format!("{}.t.json", tname));

    // 1. 检查文件是否存在
    if !fp.exists() {
        return Err(AppError::BadRequest(format!("表 {} 不存在", tname)));
    }

    // 2. 读取并反序列化
    let data = tokio::fs::read_to_string(fp).await?;
    let table: QTable = serde_json::from_str(&data)?;

    // 4. 返回正常数据
    Ok(Json(table))
}

async fn check_auth(Path(apikey): Path<String>) -> Result<impl IntoResponse, AppError> {
    // 获取请求头中的 x-api-key

    let config = CFG.get().expect("配置未初始化");

    if config.keys.contains(&apikey.to_string()) {
        return Ok("Success");
    }

    return Err(AppError::UnAuth("Uncorrect apikey".to_string()));
}

async fn save_table(Json(table): Json<QTable>) -> Result<impl IntoResponse, AppError> {
    // 1. 确保基础目录存在 (使用 create_dir_all 更安全)
    let base_path = std::path::Path::new(TDIR);
    if !base_path.exists() {
        create_dir_all(base_path).await.map_err(|e| {
            eprintln!("Failed to create TDIR: {}", e);
            AppError::from(e)
        })?;
    }

    // 2. 构造主配置文件路径: TDIR/{name}.t.json
    let file_path = base_path.join(format!("{}.t.json", &table.name));

    // 3. 构造该问卷专属的数据文件夹 (用于存放之后的回答记录)
    let table_data_dir = base_path.join(&table.name);
    if !table_data_dir.exists() {
        create_dir_all(&table_data_dir).await?;
    }

    // 4. 序列化数据
    let content = to_string(&table).map_err(|e| {
        eprintln!("Serialization error for {}: {}", table.name, e);
        AppError::from(e)
    })?;

    // 5. 写入文件 (使用 truncate 确保清空旧内容)
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // 重要：确保覆盖时不留下残余内容
        .open(&file_path)
        .await
        .map_err(|e| {
            eprintln!("File open error {}: {}", file_path.display(), e);
            AppError::from(e)
        })?;

    file.write_all(content.as_bytes()).await.map_err(|e| {
        eprintln!("File write error {}: {}", file_path.display(), e);
        AppError::from(e)
    })?;

    // 强制刷入磁盘
    file.sync_all().await?;

    println!("Successfully saved table: {}", table.name);

    Ok(Json(json!({
        "name": table.name,
        "status": "success",
        "path": file_path.to_string_lossy()
    })))
}

async fn del_table(Path(tname): Path<String>) -> Result<impl IntoResponse, AppError> {
    let bp = std::path::Path::new(TDIR);
    let backup_base = std::path::Path::new(BDIR);

    // 1. 确保备份根目录存在
    if !backup_base.exists() {
        tokio::fs::create_dir_all(backup_base).await?;
    }

    // --- 处理模板文件 (.t.json) ---
    let t_file_name = format!("{}.t.json", tname);
    let src_t_file = bp.join(&t_file_name);
    let dst_t_file = backup_base.join(&t_file_name);

    if src_t_file.exists() {
        // 使用 rename 进行移动（相当于剪切），如果跨分区可能会失败，但在大多数容器/系统内是原子操作
        tokio::fs::rename(src_t_file, dst_t_file).await?;
    } else {
        return Err(AppError::BadRequest(format!("模板文件 {} 不存在", tname)));
    }

    // --- 处理记录文件夹 (name/) ---
    let src_record_dir = bp.join(&tname);
    let dst_record_dir = backup_base.join(&tname);

    if src_record_dir.exists() {
        // 如果备份目录已存在同名文件夹，rename 会报错，所以先清理或处理
        if dst_record_dir.exists() {
            // 这里选择删除旧的备份，或者你可以加上时间戳后缀
            tokio::fs::remove_dir_all(&dst_record_dir).await?;
        }
        tokio::fs::rename(src_record_dir, dst_record_dir).await?;
    }

    Ok(StatusCode::OK)
}

async fn save_record(Json(atb): Json<Value>) -> Result<impl IntoResponse, AppError> {
    let bp = std::path::Path::new(TDIR);
    println!("{}", atb);
    let atb: TableAnswer = from_value(atb).unwrap();
    // 1. 检查模板文件夹是否存在 (docs/{name})
    // 注意：create_table 时应该已经创建了，但为了健壮性，这里再次确保
    let table_dir = bp.join(&atb.name);
    if !table_dir.exists() {
        tokio::fs::create_dir_all(&table_dir).await?;
    }

    // 2. 生成唯一 ID 和 时间戳
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Local::now().to_rfc3339();

    // 3. 构建完整的存储结构
    let record = SaveRecord {
        date: now,
        id: id.clone(),
        who: atb.who.clone(),
        // 这里需要注意：SaveRecord 里的 questions 类型是 Question
        // 如果 ATable 里的 AQuestion 和 Question 结构不一致，需要做转换
        // 下面假设你希望保存的是用户提交的原始回答
        answers: atb.answers.clone(),
    };

    // 4. 写入文件 docs/{name}/{id}.r.json
    let fp = table_dir.join(format!("{}.r.json", id));
    let content = serde_json::to_string_pretty(&record)?;
    tokio::fs::write(fp, content).await?;

    // 5. 返回生成的 ID 给前端
    Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id }))))
}

// 路由建议: .route("/record/:tname/:id", delete(del_one))
async fn del_record(
    Path((tname, id)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let fp = std::path::Path::new(TDIR)
        .join(&tname)
        .join(format!("{}.r.json", id));

    if !fp.exists() {
        return Err(AppError::BadRequest(format!("记录 {} 不存在", id)));
    }

    tokio::fs::remove_file(fp).await?;

    Ok(StatusCode::NO_CONTENT) // 204 No Content 是删除成功的标准回复
}

async fn edit_record(
    Path((tname, id)): Path<(String, String)>,
    Json(atb): Json<TableAnswer>,
) -> Result<impl IntoResponse, AppError> {
    let fp = std::path::Path::new(TDIR)
        .join(&tname)
        .join(format!("{}.r.json", id));

    if !fp.exists() {
        return Err(AppError::BadRequest(format!(
            "无法修改，记录 {} 不存在",
            id
        )));
    }

    // 1. 读取原文件以获取原始日期（或者更新日期，取决于你的需求）
    let old_data = tokio::fs::read_to_string(&fp).await?;
    let old_record: serde_json::Value = serde_json::from_str(&old_data)?;
    let created_at = old_record["date"].as_str().unwrap_or("unknown").to_string();

    // 2. 构建更新后的记录
    let updated_record = serde_json::json!({
        "id": id,
        "date": created_at, // 保留创建时间
        "edit_date": chrono::Local::now().to_rfc3339(), // 添加修改时间
        "name": atb.name,
        "who": atb.who,
        "answers": atb.answers
    });

    // 3. 覆盖写入
    let content = serde_json::to_string_pretty(&updated_record)?;
    tokio::fs::write(fp, content).await?;

    Ok(StatusCode::OK)
}

async fn get_records(Path(tname): Path<String>) -> Result<impl IntoResponse, AppError> {
    let bp = std::path::Path::new(TDIR);
    let fp = bp.join(tname);
    let mut rvec = Vec::new();
    let mut rdir = read_dir(fp).await?;
    while let Ok(Some(entry)) = rdir.next_entry().await {
        let path = entry.path();
        println!("{:?}", path);
        if !path.to_string_lossy().ends_with("r.json") {
            continue;
        }
        let str = read_to_string(entry.path()).await?;
        let r: SaveRecord = from_str(&str)?;
        rvec.push(r);
    }
    Ok(Json(rvec))
}

#[derive(Serialize, Deserialize, Clone)]
struct Who {
    name: String,
    identity: String,
    ext: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct QTable {
    enable: bool,
    name: String,
    title: String,
    know_who: bool,
    questions: Vec<Question>,
}

#[derive(Serialize, Deserialize)]
struct Question {
    required: bool,
    question: String,
    choice: Choice,
}

#[derive(Serialize, Deserialize)]
enum Choice {
    Onlyone { options: Vec<String> },
    Multiple { options: Vec<String> },
    Ask { notification: String },
}

#[derive(Serialize, Deserialize)]
struct TableAnswer {
    name: String,
    who: Option<Who>,
    answers: Vec<Answer>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Answer {
    question: String,
    choice: ChoiceAnswer,
}

#[derive(Serialize, Deserialize, Clone)]
enum ChoiceAnswer {
    Onlyone {
        options: Vec<String>,
        answer: Option<u16>,
    },
    Multiple {
        options: Vec<String>,
        answer: Vec<u16>,
    },
    Ask {
        answer: Option<String>,
    },
}

#[derive(Serialize, Deserialize)]
struct SaveRecord {
    date: String,
    id: String,
    who: Option<Who>,
    answers: Vec<Answer>,
}

// --- 错误处理模块 ---
mod ax_error {
    use super::*;

    pub enum AppError {
        Io(std::io::Error),
        Json(serde_json::Error),
        BadRequest(String),
        UnAuth(String),
    }

    impl From<std::io::Error> for AppError {
        fn from(e: std::io::Error) -> Self {
            Self::Io(e)
        }
    }

    impl From<serde_json::Error> for AppError {
        fn from(e: serde_json::Error) -> Self {
            Self::Json(e)
        }
    }

    impl IntoResponse for AppError {
        fn into_response(self) -> Response {
            let (status, msg) = match self {
                AppError::Io(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
                AppError::Json(e) => (StatusCode::BAD_REQUEST, format!("JSON Error: {}", e)),
                AppError::BadRequest(s) => (StatusCode::BAD_REQUEST, s),
                AppError::UnAuth(s) => (StatusCode::UNAUTHORIZED, s),
            };
            (status, Json(serde_json::json!({ "error": msg }))).into_response()
        }
    }
}
