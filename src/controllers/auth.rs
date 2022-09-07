use crate::{
    config::database::mongodb,
    shared::{jwt_schema::JwtSchema, user_schema::User},
};
use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::doc;
use serde_json::json;
use std::env;
// use validator::Validate;

///create a new user
pub async fn sign_up(Json(payload): Json<User>) -> impl IntoResponse {
    //destructure the request
    let User {
        username, password, ..
    } = payload;
    let database = mongodb().await;
    let collection = database.collection::<User>("user");

    //TODO: validate the user object, first check if user with username already exists
    // let error: Vec<String>;
    /*  if assert_eq!(firstname.is_empty(), true) {
        error.push("Firstname cannot be empty".to_string());
    } */

    /*
     * find user by username
     * if user already exist send error message
     * else create a new account with provided details
     */
    let user_already_exists = collection
        .find_one(doc! { "username": &username }, None)
        .await
        .unwrap();
    if let Some(_) = user_already_exists {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "success":false,
                "message":"a user with provided mail already exits",
                "data":None::<User>
            })),
        );
    }

    //construct a new user form the validated request payload
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    let user = User {
        username: username,
        password: hashed_password,
    };

    //create new user
    collection.insert_one(&user, None).await.unwrap();
    (
        StatusCode::CREATED,
        Json(json!({
            "success":true,
            "message":"user successfully created".to_string(),
            "data":None::<User>
        })),
    )
}

///login a new user
pub async fn login(Json(payload): Json<User>) -> impl IntoResponse {
    //TODO:// validate the request
    //destructure the request body
    let User {
        username,
        password: user_password,
        ..
    } = payload;

    //find user by username
    let database = mongodb().await;
    let collection = database.collection::<User>("user_information");
    let result = collection
        .find_one(doc! { "username": &username }, None)
        .await
        .unwrap();

    //try to destructure the found object
    let (username, password) = if let Some(User {
        username, password, ..
    }) = result
    {
        (username, password)
    } else {
        //if no user was found return 404 error
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success":false,
                "message":"no use with provided credentials was found".to_string(),
                "data":None::<User>
            })),
        );
    };

    //check for correctness of password, if correct send access token
    let check_password = verify(user_password, &password);
    match check_password {
        Ok(is_correct_password) => {
            //if user is found but the password is wrong, return error
            if !is_correct_password {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "success":true,
                        "message":format!("Incorrect password for {}", &username),
                        "data":None::<User>
                    })),
                );
            }

            //if user is found, fetch the JWT secret and prepare a payload
            let jwt_payload = JwtSchema { username };
            let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
                "Ux6qlTEMdT0gSLq9GHp812R9XP3KSGSWcyrPpAApy65S4NdUjioWypsTpRHxvLqYkeYNYfRZjL9"
                    .to_string()
            });
            //prepare a token
            let token = encode(
                &Header::default(),
                &jwt_payload,
                &EncodingKey::from_secret(jwt_secret.as_bytes()),
            )
            .unwrap();

            //send the response
            (
                StatusCode::OK,
                Json(json!({
                    "success":true,
                    "message":String::from("user successfully logged in"),
                    "data":json!({
                        "token":token,
                        "type":String::from("Bearer")
                    })
                })),
            )
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success":false,
                "message":String::from("Internal server error"),
                "data":None::<User>
            })),
        ),
    }
}

///reset user password
pub async fn reset_password(Json(payload): Json<User>) -> impl IntoResponse {
    //destructure the request body
    let User { username, .. } = payload;
    Json(json!({
        "username":username,
    }))
}

//get the user profile
pub async fn user_profile(Json(_payload): Json<User>) -> impl IntoResponse {}

//update user profile
pub async fn update_user_profile(Json(_payload): Json<User>) -> impl IntoResponse {}
