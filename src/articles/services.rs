use super::models::{CreateArticle, UpdateArticle};
use crate::{AppState, ArticleEntry};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/articles/list")]
async fn get_articles(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.articles_list.lock().unwrap().to_vec())
}

#[post("/articles/list")]
async fn create_article(
    data: web::Data<AppState>,
    param_obj: web::Json<CreateArticle>,
) -> impl Responder {
    let mut articles_list = data.articles_list.lock().unwrap();
    let mut max_id: i32 = 0;
    for i in 0..articles_list.len() {
        if articles_list[i].id > max_id {
            max_id = articles_list[i].id;
        }
    }

    articles_list.push(ArticleEntry {
        id: max_id + 1,
        title: param_obj.title.clone(),
        data: param_obj.data.clone(),
    });

    HttpResponse::Ok().json(articles_list.to_vec())
}

#[put("/articles/list/{id}")]
async fn update_article(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    param_obj: web::Json<UpdateArticle>,
) -> impl Responder {
    let id = path.into_inner();
    let mut articles_list = data.articles_list.lock().unwrap();
    for i in 0..articles_list.len() {
        if articles_list[i].id == id {
            articles_list[i].data = param_obj.data.clone();
            break;
        }
    }

    HttpResponse::Ok().json(articles_list.to_vec())
}

#[delete("/articles/list/{id}")]
async fn delete_article(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut articles_list = data.articles_list.lock().unwrap();
    let id = path.into_inner();
    *articles_list = articles_list
        .to_vec()
        .into_iter()
        .filter(|x| x.id != id)
        .collect();

    HttpResponse::Ok().json(articles_list.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_articles)
        .service(create_article)
        .service(update_article)
        .service(delete_article);
}
