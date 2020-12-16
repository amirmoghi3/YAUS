use crate::dto::url::create::CreateURLDTO;
use crate::dto::url::expire_with_click_creation::ExpireWithClickCreationDTO;
use crate::dto::url::expire_with_time_creation::ExpireWithTimeCreationDTO;
use crate::model::response::Response;
use actix_web::{get, post, web, HttpResponse, Responder};
use bson::{doc, Bson};
use chrono::prelude::*;
use dotenv;
use futures::stream::StreamExt;
use mongodb::Client;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;
lazy_static! {
    static ref MONGO_DB: String = dotenv::var("DATABASE_MONGO").unwrap();
    static ref URL_NOTFOUND: String = dotenv::var("URL_NOTFOUND").unwrap();
    static ref MONGO_COLLECTION: String = dotenv::var("COLLECTION_MONGO").unwrap();
    static ref PAGE_404: String = dotenv::var("URL_NOTFOUND").unwrap();
    static ref DOMAIN: String = dotenv::var("DOMAIN").unwrap();
}

#[get("/{code}")]
async fn redirect(
    data: web::Data<Mutex<Client>>,
    web::Path(code): web::Path<String>,
) -> impl Responder {
    let shortene_collection = data
        .lock()
        .unwrap()
        .database(&MONGO_DB)
        .collection(&MONGO_COLLECTION);

    let filter = doc! {"mirror":&code};

    let mut cursor = shortene_collection
        .find(filter.clone(), None)
        .await
        .unwrap();
    let update = doc! {"$inc" : {"use":1}};
    shortene_collection
        .update_one(filter, update, None)
        .await
        .unwrap();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                if let Some(url) = document.get("url").and_then(Bson::as_str) {
                    if code.len() == 10 {
                        if let Some(limit) = document.get("limit").and_then(Bson::as_i64) {
                            if let Some(used) = document.get("use").and_then(Bson::as_i64) {
                                if limit > used {
                                    return HttpResponse::Found().header("Location", url).finish();
                                }
                            }
                        }
                    } else if code.len() == 13 {
                        if let Some(expire_at) = document.get("expire_at").and_then(Bson::as_i64) {
                            let start = SystemTime::now();
                            let since_the_epoch = start
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards");
                            let now = (since_the_epoch.as_secs() * 1000
                                + since_the_epoch.subsec_nanos() as u64 / 1_000_000)
                                as i64;

                            if expire_at > now {
                                return HttpResponse::Found().header("Location", url).finish();
                            }
                        }
                    } else {
                        return HttpResponse::Found().header("Location", url).finish();
                    }

                    return HttpResponse::Found()
                        .header("Location", &**PAGE_404)
                        .finish();
                } else {
                    return HttpResponse::Found()
                        .header("Location", &**PAGE_404)
                        .finish();
                }
            }
            Err(_e) => {
                return HttpResponse::Found()
                    .header("Location", &**PAGE_404)
                    .finish();
            }
        }
    }
    return HttpResponse::Found()
        .header("Location", &**PAGE_404)
        .finish();
}

#[post("/compress")]
async fn compressor(
    data: web::Data<Mutex<Client>>,
    url: web::Json<CreateURLDTO>,
) -> impl Responder {
    if !url.url.contains(&**DOMAIN) {
        match Url::parse(&url.url) {
            Ok(_) => (),
            Err(_e) => return HttpResponse::UnprocessableEntity().finish(),
        }
        let shortene_collection = data
            .lock()
            .unwrap()
            .database(&MONGO_DB)
            .collection(&MONGO_COLLECTION);
        let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(6).collect();
        match shortene_collection
        .insert_one(
            doc! {"url":&url.url,"mirror":&rand_string,"createdOn": Bson::DateTime(Utc::now()) ,"use":0 as i64},
            None,
        )
        .await
    {
        Ok(db_result) => {
            if let Some(_new_id) = db_result.inserted_id.as_object_id() {
                let res = Response { code: rand_string };
                return HttpResponse::Created().json(&res);
            } else {
                return HttpResponse::InternalServerError().finish();
            }
        }
        Err(_e) => {
            println!("{}",_e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    }
    return HttpResponse::NotAcceptable().finish();
}

#[post("/expand")]
async fn expander(data: web::Data<Mutex<Client>>, url: web::Json<CreateURLDTO>) -> impl Responder {
    if !url.url.contains(&**DOMAIN) {
        match Url::parse(&url.url) {
            Ok(_) => (),
            Err(_e) => return HttpResponse::UnprocessableEntity().finish(),
        }
        let shortene_collection = data
            .lock()
            .unwrap()
            .database(&MONGO_DB)
            .collection(&MONGO_COLLECTION);
        println!("{:?},{:?}", &**MONGO_DB, &**MONGO_COLLECTION);
        let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(128).collect();
        match shortene_collection
        .insert_one(
            doc! {"url":&url.url,"mirror":&rand_string,"created_on": Bson::DateTime(Utc::now()) ,"use":0 as i64},
            None,
        )
        .await
    {
        Ok(db_result) => {
            if let Some(_new_id) = db_result.inserted_id.as_object_id() {
                let res = Response { code: rand_string };
                return HttpResponse::Created().json(&res);
            } else {
                return HttpResponse::InternalServerError().finish();
            }
        }
        Err(_e) => {
            println!("{}",_e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    }
    return HttpResponse::NotAcceptable().finish();
}

#[post("/expire/click")]
async fn expire_with_click(
    data: web::Data<Mutex<Client>>,
    url: web::Json<ExpireWithClickCreationDTO>,
) -> impl Responder {
    if !url.url.contains(&**DOMAIN) {
        if url.click > 0 {
            match Url::parse(&url.url) {
                Ok(_) => (),
                Err(_e) => return HttpResponse::UnprocessableEntity().finish(),
            }
            let shortene_collection = data
                .lock()
                .unwrap()
                .database(&MONGO_DB)
                .collection(&MONGO_COLLECTION);
            println!("{:?},{:?}", &**MONGO_DB, &**MONGO_COLLECTION);
            let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
            match shortene_collection
        .insert_one(
            doc! {"url":&url.url,"mirror":&rand_string,"createdOn": Bson::DateTime(Utc::now()) ,"use":0 as i64,"limit":&url.click},
            None,
        )
        .await
    {
        Ok(db_result) => {
            if let Some(_new_id) = db_result.inserted_id.as_object_id() {
                let res = Response { code: rand_string };
                return HttpResponse::Created().json(&res);
            } else {
                return HttpResponse::InternalServerError().finish();
            }
        }
        Err(_e) => {
            println!("{}",_e);
            return HttpResponse::InternalServerError().finish();
        }
    }
        }
    }
    return HttpResponse::NotAcceptable().finish();
}

#[post("/expire/time")]
async fn expire_with_time(
    data: web::Data<Mutex<Client>>,
    url: web::Json<ExpireWithTimeCreationDTO>,
) -> impl Responder {
    if !url.url.contains(&**DOMAIN) {
        if url.expire_at > 0 && url.expire_at > 999999999999 {
            match Url::parse(&url.url) {
                Ok(_) => (),
                Err(_e) => return HttpResponse::UnprocessableEntity().finish(),
            }
            let shortene_collection = data
                .lock()
                .unwrap()
                .database(&MONGO_DB)
                .collection(&MONGO_COLLECTION);

            let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(13).collect();
            match shortene_collection
        .insert_one(
            doc! {"url":&url.url,"mirror":&rand_string,"createdOn": Bson::DateTime(Utc::now()) ,"use":0 as i64,"expire_at":&url.expire_at},
            None,
        )
        .await
    {
        Ok(db_result) => {
            if let Some(_new_id) = db_result.inserted_id.as_object_id() {
                let res = Response { code: rand_string };
                return HttpResponse::Created().json(&res);
            } else {
                return HttpResponse::InternalServerError().finish();
            }
        }
        Err(_e) => {
            return HttpResponse::InternalServerError().finish();
        }
    }
        }
    }
    return HttpResponse::NotAcceptable().finish();
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(compressor);
    cfg.service(redirect);
    cfg.service(expander);
    cfg.service(expire_with_click);
    cfg.service(expire_with_time);
    // cfg.service(user_informations);
    // cfg.service(user_informations_get);
    // cfg.service(protected);
}
