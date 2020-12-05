use crate::dto::url::create::CreateURLDTO;
use crate::model::response::Response;

use actix_web::{get, post, web, HttpResponse, Responder};
use bson::{doc, Bson};
use chrono::prelude::*;
use futures::stream::StreamExt;
use mongodb::Client;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::sync::Mutex;
const MONGO_DB: &'static str = "url";
const MONGO_COLL_LOGS: &'static str = "shorten";
use url::Url;
lazy_static! {
    static ref RE_URL: Regex = Regex::new(
        r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)"
    )
    .unwrap();
}

#[get("/{code}")]
async fn redirect(
    data: web::Data<Mutex<Client>>,
    web::Path(code): web::Path<String>,
) -> impl Responder {
    let shortene_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLL_LOGS);
    let filter = doc! {"mirror":&code};
    let mut cursor = shortene_collection.find(filter, None).await.unwrap();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                if let Some(url) = document.get("url").and_then(Bson::as_str) {
                    println!("{}", url);
                    return HttpResponse::Found().header("Location", url).finish();
                } else {
                    return HttpResponse::NotFound().finish();
                }
            }
            Err(_e) => {
                return HttpResponse::NotFound().finish();
            }
        }
    }
    return HttpResponse::NotFound().finish();
}

#[post("/link")]
async fn compress_url(
    data: web::Data<Mutex<Client>>,
    url: web::Json<CreateURLDTO>,
) -> impl Responder {
    // println!("{} , {:?}", RE_URL.is_match(&url.url), RE_URL.as_str());
    // if !RE_URL.is_match(&url.url) {
    //     return HttpResponse::UnprocessableEntity().finish();
    // }

    match Url::parse(&url.url) {
        Ok(_) => (),
        Err(_e) => return HttpResponse::UnprocessableEntity().finish(),
    }
    let shortene_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLL_LOGS);
    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(6).collect();
    match shortene_collection
        .insert_one(
            doc! {"url":&url.url,"mirror":&rand_string,"createdOn": Bson::DateTime(Utc::now())},
            None,
        )
        .await
    {
        Ok(db_result) => {
            if let Some(new_id) = db_result.inserted_id.as_object_id() {
                println!("New document inserted with id {} : {}", new_id, &url.url);
            }
            let res = Response {
                message: "The Short Link Created Successfuly".to_string(),
                status: true,
                code: rand_string,
            };
            return HttpResponse::Created().json(&res);
        }
        Err(err) => {
            println!("Failed! {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(compress_url);
    cfg.service(redirect);
    // cfg.service(user_informations);
    // cfg.service(user_informations_get);
    // cfg.service(protected);
}
