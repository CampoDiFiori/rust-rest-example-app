use crate::{
    models::{Post, PostCreateJson, PostUpdateJson, User, UserJson, UserNew, UserUpdateJson},
    DbPool,
};

use actix_web::error::BlockingError;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use anyhow::Result;
use diesel::dsl::{insert_into, update};
use diesel::prelude::*;
use diesel::result::Error as DieselErr;
use diesel::RunQueryDsl;

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::users;
    let dbconn = pool.get().unwrap();
    let result = web::block(move || users.load::<User>(&dbconn))
        .await
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/users/{user_id}")] // <- define path parameters
pub async fn get_user(
    pool: web::Data<DbPool>,
    web::Path(user_id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let dbconn = pool.get().unwrap();

    let user = web::block(move || users.find(user_id).get_result::<User>(&dbconn))
        .await
        .map_err(|e: BlockingError<DieselErr>| match e {
            BlockingError::Error(DieselErr::NotFound) => HttpResponse::NotFound()
                .body(format!("Couldn't find such a user with id {}", user_id)),
            e => HttpResponse::InternalServerError().body(e.to_string()),
        })?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    item: web::Json<UserJson>,
) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let dbconn = pool.get().unwrap();

    let result = web::block(move || -> Result<User, anyhow::Error> {
        let new_user = UserNew {
            name: &item.name.as_str(),
            email: &item.email.as_str(),
            created_at: &format!("{}", chrono::Local::now().naive_local()),
        };
        insert_into(users).values(&new_user).execute(&dbconn)?;
        let result = users.order(id.desc()).first(&dbconn)?;
        Ok(result)
    })
    .await
    .map(|user| HttpResponse::Created().json(user))
    .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))?;

    Ok(result)
}

#[put("/users")]
pub async fn update_user(
    pool: web::Data<DbPool>,
    user_json: web::Json<UserUpdateJson>,
) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let dbconn1 = pool.get().unwrap();
    let dbconn2 = pool.get().unwrap();

    let user_id = user_json.id;

    let _user = web::block(move || users.find(user_id).get_result::<User>(&dbconn1))
        .await
        .map_err(|e: BlockingError<DieselErr>| match e {
            BlockingError::Error(DieselErr::NotFound) => HttpResponse::NotFound()
                .body(format!("Couldn't find such a user with id {}", user_id)),
            e => HttpResponse::InternalServerError().body(e.to_string()),
        })?;

    let updated_user = web::block(move || -> Result<User, anyhow::Error> {
        update(users.filter(id.eq(user_json.id)))
            .set((name.eq(&user_json.name), email.eq(&user_json.email)))
            .execute(&dbconn2)?;
        let result_vec = users.find(user_json.id).load::<User>(&dbconn2)?;
        let result = result_vec[0].clone();

        Ok(result)
    })
    .await
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))?;

    Ok(updated_user)
}

#[delete("/users/{user_id}")]
pub async fn delete_user(
    pool: web::Data<DbPool>,
    web::Path(user_id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let dbconn = pool.get().unwrap();

    web::block(move || diesel::delete(users.filter(id.eq(user_id))).execute(&dbconn))
        .await
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/posts")]
pub async fn get_posts(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    use crate::schema::posts::dsl::posts;
    let dbconn = pool.get().unwrap();
    let result = web::block(move || posts.load::<Post>(&dbconn))
        .await
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/posts/{post_id}")] // <- define path parameters
pub async fn get_post(
    pool: web::Data<DbPool>,
    web::Path(post_id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    use crate::schema::posts::dsl::*;
    let dbconn = pool.get().unwrap();

    let post = web::block(move || posts.find(post_id).get_result::<Post>(&dbconn))
        .await
        .map_err(|e: BlockingError<DieselErr>| match e {
            BlockingError::Error(DieselErr::NotFound) => HttpResponse::NotFound()
                .body(format!("Couldn't find such a post with id {}", post_id)),
            e => HttpResponse::InternalServerError().body(e.to_string()),
        })?;

    Ok(HttpResponse::Ok().json(post))
}

// #[post("/posts")]
pub async fn create_post<'a>(
    pool: web::Data<DbPool>,
    item: web::Json<PostCreateJson>,
) -> Result<HttpResponse, Error> {
    use crate::schema::posts::dsl::*;
    let dbconn = pool.get().unwrap();

    let result = web::block(move || -> Result<Post, anyhow::Error> {
        insert_into(posts)
            .values(&item.into_inner())
            .execute(&dbconn)?;
        let result = posts.order(id.desc()).first(&dbconn)?;
        Ok(result)
    })
    .await
    .map(|user| HttpResponse::Created().json(user))
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(result)
}

async fn update_post_helper(
    pool: web::Data<DbPool>,
    post_id: i32,
    post_title: String,
    post_user_id: Option<i32>,
) -> Result<Post, actix_web::error::Error> {
    use crate::schema::posts::dsl::*;

    let dbconn = pool.get().unwrap();

    let updated_post = web::block(move || -> Result<Post, DieselErr> {
        update(posts.filter(id.eq(post_id)))
            .set((title.eq(&post_title), user_id.eq(&post_user_id)))
            .execute(&dbconn)?;
        let updated_post = posts.find(post_id).get_result::<Post>(&dbconn)?;

        Ok(updated_post)
    })
    .await
    .map_err(|e: BlockingError<DieselErr>| match e {
        BlockingError::Error(DieselErr::NotFound) => {
            HttpResponse::NotFound().body(format!("Couldn't find such a post with id {}", post_id))
        }
        e => HttpResponse::InternalServerError().body(e.to_string()),
    })?;

    Ok(updated_post)
}

// #[put("/posts")]
pub async fn update_post(
    pool: web::Data<DbPool>,
    post_json: web::Json<PostUpdateJson>,
) -> Result<HttpResponse, Error> {
    let updated_post = update_post_helper(
        pool,
        post_json.id,
        post_json.title.clone(),
        post_json.user_id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(updated_post))
}

#[delete("/posts/{post_id}")]
pub async fn delete_post(
    pool: web::Data<DbPool>,
    web::Path(post_id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    use crate::schema::posts::dsl::*;
    let dbconn = pool.get().unwrap();

    web::block(move || diesel::delete(posts.filter(id.eq(post_id))).execute(&dbconn))
        .await
        .map_err(|_e| HttpResponse::InternalServerError().finish())?;
    Ok(HttpResponse::NoContent().finish())
}

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct UserWithPosts {
    user: User,
    posts: Vec<Post>,
}

#[derive(Serialize)]
struct Posts {
    posts: Vec<Post>,
}

async fn get_user_with_posts_helper(
    pool: web::Data<DbPool>,
    user_id: i32,
) -> Result<(User, Vec<Post>), Error> {
    use crate::schema::users::dsl::*;

    let dbconn1 = pool.get().unwrap();
    let dbconn2 = pool.get().unwrap();

    let user = web::block(move || users.find(user_id).get_result::<User>(&dbconn1))
        .await
        .map_err(|e: BlockingError<DieselErr>| match e {
            BlockingError::Error(DieselErr::NotFound) => HttpResponse::NotFound()
                .body(format!("Couldn't find such a user with id {}", user_id)),
            _ => HttpResponse::InternalServerError().finish(),
        })?;

    let user_cl = user.clone();

    let posts = web::block(move || Post::belonging_to(&user_cl).load::<Post>(&dbconn2)).await?;

    return Ok((user, posts));
}

#[get("/user_with_posts/{user_id}")]
pub async fn get_user_with_posts(
    pool: web::Data<DbPool>,
    web::Path(user_id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let (user, posts) = get_user_with_posts_helper(pool, user_id).await?;
    Ok(HttpResponse::Ok().json(UserWithPosts { user, posts }))
}

#[get("/users_posts/{user_id}")]
pub async fn get_users_posts(
    pool: web::Data<DbPool>,
    web::Path(user_id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let (_user, posts) = get_user_with_posts_helper(pool, user_id).await?;
    Ok(HttpResponse::Ok().json(Posts { posts }))
}

#[derive(Deserialize)]
pub struct UpdatePostAuthorJson {
    post_id: i32,
    user_id: i32,
}

// #[put("/link_post_author")]
pub async fn link_post_author(
    pool: web::Data<DbPool>,
    request_json: web::Json<UpdatePostAuthorJson>,
) -> Result<HttpResponse, Error> {
    use crate::schema::posts::dsl::*;
    let dbconn = pool.get().unwrap();

    let post_id = request_json.post_id;

    let post: Post = web::block(move || posts.find(post_id).get_result::<Post>(&dbconn))
        .await
        .map_err(|e: BlockingError<DieselErr>| match e {
            BlockingError::Error(DieselErr::NotFound) => HttpResponse::NotFound()
                .body(format!("Couldn't find such a post with id {}", post_id)),
            e => HttpResponse::InternalServerError().body(e.to_string()),
        })?;

    let updated_post = update_post_helper(
        pool,
        post.id,
        post.title.clone(),
        Some(request_json.user_id),
    )
    .await?;

    Ok(HttpResponse::Ok().json(updated_post))
}

#[put("/unlink_post_author/{post_id}")]
pub async fn unlink_post_author(
    pool: web::Data<DbPool>,
    web::Path(post_id): web::Path<i32>,
) -> Result<HttpResponse, Error> {
    use crate::schema::posts::dsl::*;
    let dbconn = pool.get().unwrap();

    let post: Post = web::block(move || posts.find(post_id).get_result::<Post>(&dbconn))
        .await
        .map_err(|e: BlockingError<DieselErr>| match e {
            BlockingError::Error(DieselErr::NotFound) => HttpResponse::NotFound()
                .body(format!("Couldn't find such a post with id {}", post_id)),
            e => HttpResponse::InternalServerError().body(e.to_string()),
        })?;

    let updated_post = update_post_helper(pool, post.id, post.title.clone(), None).await?;

    Ok(HttpResponse::Ok().json(updated_post))
}
