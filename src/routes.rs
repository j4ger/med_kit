use std::ops::Deref;

use crate::{profile::Profile, AppState};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use mongodb::{
    bson,
    bson::{doc, Bson},
};
use nanoid::nanoid;
use serde::Serialize;

#[derive(Serialize)]
struct STDJSONResponse<T: Serialize> {
    success: bool,
    data: Option<T>,
}

#[derive(Serialize)]
struct CreateData {
    uuid: String,
}

#[derive(Serialize)]
struct GetData {
    exist: bool,
    init: bool,
    profile: Option<Profile>,
}

#[derive(Serialize)]
struct UpdateData {
    exist: bool,
    init: bool,
    updated_count: i64,
}

#[derive(Serialize)]
struct InitData {
    exist: bool,
}

fn gen_std_json_response<T: Serialize>(data: Option<T>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(STDJSONResponse {
            success: data.is_some(),
            data,
        })
}

#[get("/new")]
pub async fn new_prod(app_state: web::Data<AppState>) -> impl Responder {
    let mut uuid = Local::now().format("%Y%m%d%H%M").to_string();
    uuid.push_str(&nanoid!(
        4,
        &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
    ));

    match app_state
        .profile
        .create(doc! {"uuid":&uuid,"init":false,"delete":false})
        .await
    {
        Ok(_) => gen_std_json_response(Some(CreateData { uuid: uuid })),
        Err(err) => {
            println!("Err:{:?}", err);
            gen_std_json_response(None)
        }
    }
}

#[get("/init/{uuid}")]
pub async fn init_prod(
    app_state: web::Data<AppState>,
    web::Path(uuid): web::Path<String>,
) -> impl Responder {
    // TODO: uuid验证（可能的查询字符串注入？）
    match app_state.profile.get(doc! {"uuid":&uuid}).await {
        Ok(result) => match result {
            Some(_) => gen_std_json_response(Some(InitData { exist: true })),
            None => match app_state
                .profile
                .create(doc! {"uuid":&uuid,"init":false})
                .await
            {
                Ok(_) => gen_std_json_response(Some(InitData { exist: false })),
                Err(err) => {
                    println!("Err:{:?}", err);
                    gen_std_json_response(None)
                }
            },
        },
        Err(err) => {
            println!("Err:{:?}", err);
            gen_std_json_response(None)
        }
    }
}

#[get("/fetch/{uuid}")]
pub async fn fetch(
    app_state: web::Data<AppState>,
    web::Path(uuid): web::Path<String>,
) -> impl Responder {
    // TODO: uuid验证（可能的查询字符串注入？）
    match app_state.profile.get(doc! {"uuid":&uuid}).await {
        Ok(result) => {
            match result {
                // TODO: 错误处理
                Some(inner) => {
                    let init = inner.get("init").and_then(Bson::as_bool).unwrap();
                    return gen_std_json_response(Some(GetData {
                        exist: true,
                        init: inner.get("init").and_then(Bson::as_bool).unwrap(),
                        profile: if init {
                            Some(bson::from_bson(Bson::Document(inner.clone())).unwrap())
                        } else {
                            None
                        },
                    }));
                }
                None => gen_std_json_response(Some(GetData {
                    exist: false,
                    init: false,
                    profile: None,
                })),
            }
        }
        Err(err) => {
            println!("Err:{:?}", err);
            return gen_std_json_response(None);
        }
    }
}

#[post("/submit")]
pub async fn submit(
    app_state: web::Data<AppState>,
    form_data: web::Json<Profile>,
) -> impl Responder {
    // TODO: uuid验证（可能的查询字符串注入？）
    let parsed_data = form_data.deref();
    let uuid = &parsed_data.uuid;
    match app_state.profile.get(doc! {"uuid":&uuid}).await {
        Ok(result) => match result {
            Some(inner) => {
                if inner.get("init").and_then(Bson::as_bool).unwrap() {
                    gen_std_json_response(Some(UpdateData {
                        exist: true,
                        init: true,
                        updated_count: 0,
                    }))
                } else {
                    match app_state
                        .profile
                        .set(doc! {"uuid":&uuid}, bson::to_document(parsed_data).unwrap())
                        .await
                    {
                        Ok(result) => gen_std_json_response(Some(UpdateData {
                            exist: true,
                            init: false,
                            updated_count: result.modified_count,
                        })),
                        Err(err) => {
                            println!("Err:{:?}", err);
                            return gen_std_json_response(None);
                        }
                    }
                }
            }
            None => gen_std_json_response(Some(UpdateData {
                exist: false,
                init: false,
                updated_count: 0,
            })),
        },
        Err(err) => {
            println!("Err:{:?}", err);
            return gen_std_json_response(None);
        }
    }
}
