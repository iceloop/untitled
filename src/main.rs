
extern crate dotenv;

use dotenv::dotenv;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware};
use sqlx::mysql::MySqlPool;
use std::env;

mod models;

use models::{Article, NewArticle, UpdateArticle};

async fn get_all_article(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Article>("SELECT * FROM articles")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(e) => {
            eprintln!("Error querying the database: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


async fn get_article_by_id(
    id: web::Path<i32>,
    pool: web::Data<MySqlPool>
) -> impl Responder {
    let article_id = id.into_inner();

    match sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
        .bind(article_id)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(_) => HttpResponse::NotFound().into(),
    }
}

async fn add_article(
    new_article: web::Json<NewArticle>,
    pool: web::Data<MySqlPool>
) -> impl Responder {
    let article = new_article.into_inner();

    match sqlx::query("INSERT INTO articles (title, content) VALUES (?, ?)")
        .bind(&article.title)
        .bind(&article.content)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Created().finish(), // 直接使用 .finish()
        Err(_) => HttpResponse::InternalServerError().finish(), // 同理
    }
}

async fn update_article(
    id: web::Path<i32>,
    article_data: web::Json<UpdateArticle>,
    pool: web::Data<MySqlPool>
) -> impl Responder {
    let article_id = id.into_inner();
    let article = article_data.into_inner();

    match sqlx::query("UPDATE articles SET title = ?, content = ? WHERE id = ?")
        .bind(&article.title)
        .bind(&article.content)
        .bind(article_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Created().finish(), // 直接使用 .finish()
        Err(_) => HttpResponse::InternalServerError().finish(), // 同理
    }
}

async fn delete_article(
    id: web::Path<i32>,
    pool: web::Data<MySqlPool>
) -> impl Responder {
    let article_id = id.into_inner();

    match sqlx::query("DELETE FROM articles WHERE id = ?")
        .bind(article_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Created().finish(), // 直接使用 .finish()
        Err(_) => HttpResponse::InternalServerError().finish(), // 同理
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPool::connect(&database_url).await.expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/Article", web::get().to(get_all_article))
            .route("/Article/{id}", web::get().to(get_article_by_id))
            .route("/Article", web::post().to(add_article))
            .route("/Article/{id}", web::put().to(update_article))
            .route("/Article/{id}", web::delete().to(delete_article))
            .wrap(middleware::Logger::default())
    })
        .bind("127.0.0.1:9090")?
        .run()
        .await
}
