use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json,
use serde_json::Value;

pub async fn list_users() -> (StatusCode, Json<Value>){
    //get users
}

pub async fn get_user_by_id(Path<u64>)