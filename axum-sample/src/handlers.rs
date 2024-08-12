use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use std::sync::Arc;
use crate::models::{CreateUserRequest, UpdateUserRequest, User};
use crate::AppState;
use futures::stream::TryStreamExt;

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let collection: Collection<User> = state.db_client.collection("users");

    let new_user = User {
        id: None,
        name: payload.name,
        mobile: payload.mobile,
        email: payload.email
    };

    match collection.insert_one(new_user, None).await {
        Ok(result) => {
            let user_id = result.inserted_id.as_object_id().unwrap();
            let created_user = collection
                .find_one(doc! { "_id": user_id }, None)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch created user".to_string()))?
                .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Created user not found".to_string()))?;

            Ok((StatusCode::CREATED, Json(created_user)))
        }
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user".to_string())),
    }
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<User>, (StatusCode, String)> {
    let collection: Collection<User> = state.db_client.collection("users");
    let object_id = ObjectId::parse_str(&id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ID".to_string()))?;

    match collection.find_one(doc! { "_id": object_id }, None).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch user".to_string())),
    }
}

pub async fn get_all_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let collection: Collection<User> = state.db_client.collection("users");

    match collection.find(None, None).await {
        Ok(cursor) => {
            let users: Vec<User> = cursor.try_collect().await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch users".to_string()))?;
            Ok(Json(users))
        }
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch users".to_string())),
    }
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    let collection: Collection<User> = state.db_client.collection("users");
    let object_id = ObjectId::parse_str(&id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ID".to_string()))?;

    let update = doc! {
        "$set": {
            "name": payload.name,
            "email": payload.email,
        }
    };

    match collection.find_one_and_update(doc! { "_id": object_id }, update, None).await {
        Ok(Some(updated_user)) => Ok(Json(updated_user)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to update user".to_string())),
    }
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let collection: Collection<User> = state.db_client.collection("users");
    let object_id = ObjectId::parse_str(&id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ID".to_string()))?;

    match collection.delete_one(doc! { "_id": object_id }, None).await {
        Ok(result) if result.deleted_count == 1 => Ok(StatusCode::NO_CONTENT),
        Ok(_) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete user".to_string())),
    }
}