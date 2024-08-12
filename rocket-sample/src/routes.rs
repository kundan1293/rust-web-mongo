use crate::models::{User, CreateUserRequest, UpdateUserRequest};
use crate::AppState;
use mongodb::bson::{doc, oid::ObjectId};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};
use futures::stream::TryStreamExt;


#[post("/users", data = "<new_user>")]
pub async fn create_user(state: &State<AppState>, new_user: Json<CreateUserRequest>) -> Result<Json<User>, Status> {
    let collection = state.db_client.collection::<User>("users");
    let user = User {
        id: None,
        name: new_user.name.clone(),
        mobile: new_user.mobile.clone(),
        email: new_user.email.clone(), 
    };

    let insert_result = collection.insert_one(user, None).await;
    match insert_result {
        Ok(result) => {
            let new_id = result.inserted_id.as_object_id().unwrap();
            let created_user = collection.find_one(doc! { "_id": new_id }, None).await.unwrap().unwrap();
            Ok(Json(created_user))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users/<id>")]
pub async fn get_user(state: &State<AppState>, id: String) -> Result<Json<User>, Status> {
    let collection = state.db_client.collection::<User>("users");
    let object_id = ObjectId::parse_str(&id).map_err(|_| Status::BadRequest)?;

    match collection.find_one(doc! { "_id": object_id }, None).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users")]
pub async fn get_all_users(state: &State<AppState>) -> Result<Json<Vec<User>>, Status> {
    let collection = state.db_client.collection::<User>("users");

    match collection.find(None, None).await {
        Ok(cursor) => {
            let users: Vec<User> = cursor.try_collect().await.map_err(|_| Status::InternalServerError)?;
            Ok(Json(users))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/users/<id>", data = "<user_update>")]
pub async fn update_user(state: &State<AppState>, id: String, user_update: Json<UpdateUserRequest>) -> Result<Json<User>, Status> {
    let collection = state.db_client.collection::<User>("users");
    let object_id = ObjectId::parse_str(&id).map_err(|_| Status::BadRequest)?;

    let update = doc! {
        "$set": {
            "name": &user_update.name,
            "email": &user_update.email,
        }
    };

    match collection.find_one_and_update(doc! { "_id": object_id }, update, None).await {
        Ok(Some(updated_user)) => Ok(Json(updated_user)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/users/<id>")]
pub async fn delete_user(state: &State<AppState>, id: String) -> Status {
    let collection = state.db_client.collection::<User>("users");
    let object_id = match ObjectId::parse_str(&id) {
        Ok(object_id) => object_id,
        Err(_) => return Status::BadRequest,
    };

    match collection.delete_one(doc! { "_id": object_id }, None).await {
        Ok(result) if result.deleted_count == 1 => Status::NoContent,
        Ok(_) => Status::NotFound,
        Err(_) => Status::InternalServerError,
    }
}