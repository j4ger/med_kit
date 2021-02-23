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
    updated_count: i64,
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
        .create(doc! {"uuid":&uuid,"init":false})
        .await
    {
        Ok(_) => HttpResponse::Created()
            .content_type("application/json")
            .json(STDJSONResponse {
                success: true,
                data: Some(CreateData { uuid: uuid }),
            }),
        Err(err) => {
            println!("Err:{:?}", err);
            HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(STDJSONResponse::<CreateData> {
                    success: false,
                    data: None,
                })
        }
    }
}

#[get("/fetch/{uuid}")]
pub async fn fetch(
    app_state: web::Data<AppState>,
    web::Path(uuid): web::Path<String>,
) -> impl Responder {
    match app_state.profile.get(doc! {"uuid":uuid}).await {
        Ok(result) => {
            match result {
                // TODO: 错误处理
                Some(inner) => {
                    let init = inner.get("init").and_then(Bson::as_bool).unwrap();
                    return HttpResponse::Ok().content_type("application/json").json(
                        STDJSONResponse {
                            success: true,
                            data: Some(GetData {
                                exist: true,
                                init: inner.get("init").and_then(Bson::as_bool).unwrap(),
                                profile: if init {
                                    Some(bson::from_bson(Bson::Document(inner.clone())).unwrap())
                                } else {
                                    None
                                },
                            }),
                        },
                    );
                }
                None => {
                    return HttpResponse::Ok().content_type("application/json").json(
                        STDJSONResponse {
                            success: true,
                            data: Some(GetData {
                                exist: false,
                                init: false,
                                profile: None,
                            }),
                        },
                    )
                }
            }
        }
        Err(err) => {
            println!("Err:{:?}", err);
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(STDJSONResponse::<GetData> {
                    success: false,
                    data: None,
                });
        }
    }
}

#[post("/submit")]
pub async fn submit(
    app_state: web::Data<AppState>,
    form_data: web::Json<Profile>,
) -> impl Responder {
    let parsed_data = form_data.deref();
    let uuid = &parsed_data.uuid;
    let query = app_state.profile.get(doc! {"uuid":uuid}).await;
    match query {
        Ok(result) => match result {
            Some(inner) => {
                if inner.get("init").and_then(Bson::as_bool).unwrap() {
                    HttpResponse::Ok()
                        .content_type("application/json")
                        .json(STDJSONResponse {
                            success: true,
                            data: Some(GetData {
                                exist: true,
                                init: true,
                                profile: None,
                            }),
                        })
                } else {
                    match app_state
                        .profile
                        .set(doc! {"uuid":uuid}, bson::to_document(parsed_data).unwrap())
                        .await
                    {
                        Ok(result) => HttpResponse::Ok().content_type("application/json").json(
                            STDJSONResponse {
                                success: true,
                                data: Some(UpdateData {
                                    updated_count: result.modified_count,
                                }),
                            },
                        ),
                        Err(err) => {
                            println!("Err:{:?}", err);
                            return HttpResponse::InternalServerError()
                                .content_type("application/json")
                                .json(STDJSONResponse::<GetData> {
                                    success: false,
                                    data: None,
                                });
                        }
                    }
                }
            }
            None => HttpResponse::Ok()
                .content_type("application/json")
                .json(STDJSONResponse {
                    success: true,
                    data: Some(GetData {
                        exist: false,
                        init: false,
                        profile: None,
                    }),
                }),
        },
        Err(err) => {
            println!("Err:{:?}", err);
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(STDJSONResponse::<GetData> {
                    success: false,
                    data: None,
                });
        }
    }
}
