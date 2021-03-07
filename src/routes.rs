use crate::{
    auth::{Claims, JWTAuthService},
    config::get_config,
    profile::Profile,
    user::User,
    AppState,
};
use actix_web::{
    cookie::{Cookie, SameSite},
    get,
    http::StatusCode,
    post, web, HttpMessage, HttpRequest, HttpResponse,
};
use chrono::{prelude::*, Duration, Utc};
use hmac::{Hmac, Mac, NewMac};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::{
    bson,
    bson::{doc, Bson},
};
use nanoid::nanoid;
use reqwest;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize)]
struct STDJSONResponse<T: Serialize> {
    success: bool,
    data: Option<T>,
    errmsg: Option<String>,
}

#[derive(Serialize)]
struct CreateData {
    uuid: String,
}

#[derive(Serialize)]
struct QueryData {
    exist: bool,
    init: bool,
}

#[derive(Serialize)]
struct GetData {
    profile: Profile,
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

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    pwhashed: String,
}

#[derive(Serialize)]
struct VerifyData {
    username: String,
}

#[derive(Serialize)]
struct LoginData {
    JWT: bool,
}

#[derive(Serialize)]
struct RegisterData {
    username: String,
}

#[derive(Serialize, Deserialize)]
struct OpenIDResponse {
    openid: Option<String>,
    session_key: Option<String>,
    unionid: Option<String>,
    errcode: i64,
    errmsg: String,
    hint: Option<String>,
}

pub fn gen_std_json_response<T: Serialize>(
    data: Option<T>,
    errmsg: Option<String>,
) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(STDJSONResponse {
            success: data.is_some(),
            data,
            errmsg,
        })
}

#[get("/new")]
pub async fn new_product(app_state: web::Data<AppState>, _: JWTAuthService) -> HttpResponse {
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
        Ok(_) => gen_std_json_response(Some(CreateData { uuid: uuid }), None),
        Err(err) => {
            println!("Err:{:?}", err);
            gen_std_json_response::<CreateData>(None, Some("服务器插入错误！".to_string()))
        }
    }
}

#[post("/init/{uuid}")]
pub async fn init_product(
    app_state: web::Data<AppState>,
    web::Path(uuid): web::Path<String>,
    _: JWTAuthService,
) -> HttpResponse {
    //TODO: csrf token

    // TODO: uuid验证（可能的查询字符串注入？）
    match app_state.profile.get(doc! {"uuid":&uuid}).await {
        Ok(result) => match result {
            Some(_) => gen_std_json_response(Some(InitData { exist: true }), None),
            None => match app_state
                .profile
                .create(doc! {"uuid":&uuid,"init":false})
                .await
            {
                Ok(_) => gen_std_json_response(Some(InitData { exist: false }), None),
                Err(err) => {
                    println!("Err:{:?}", err);
                    gen_std_json_response::<InitData>(None, Some("数据库插入错误！".to_string()))
                }
            },
        },
        Err(err) => {
            println!("Err:{:?}", err);
            gen_std_json_response::<InitData>(None, Some("数据库查询错误！".to_string()))
        }
    }
}

#[get("/query/{uuid}")]
pub async fn query_profile(
    app_state: web::Data<AppState>,
    web::Path(uuid): web::Path<String>,
) -> HttpResponse {
    // TODO: uuid验证（可能的查询字符串注入？）
    match app_state.profile.get(doc! {"uuid":&uuid}).await {
        Ok(result) => {
            match result {
                // TODO: 错误处理
                Some(inner) => {
                    return gen_std_json_response(
                        Some(QueryData {
                            exist: true,
                            init: inner.get("init").and_then(Bson::as_bool).unwrap(),
                        }),
                        None,
                    );
                }
                None => gen_std_json_response(
                    Some(QueryData {
                        exist: false,
                        init: false,
                    }),
                    None,
                ),
            }
        }
        Err(err) => {
            println!("Err:{:?}", err);
            return gen_std_json_response::<QueryData>(None, Some("服务器查询错误！".to_string()));
        }
    }
}

#[get("/get/{uuid}")]
pub async fn get_profile(
    app_state: web::Data<AppState>,
    web::Path(uuid): web::Path<String>,
    _: JWTAuthService,
) -> HttpResponse {
    match app_state.profile.get(doc! {"uuid":&uuid}).await {
        Ok(result) => match result {
            Some(inner) => {
                if inner.get("init").and_then(Bson::as_bool).unwrap() {
                    let parsed_profile: Profile = bson::from_document(inner).unwrap();
                    return gen_std_json_response(
                        Some(GetData {
                            profile: parsed_profile,
                        }),
                        None,
                    );
                } else {
                    return gen_std_json_response::<GetData>(
                        None,
                        Some("档案未填写！".to_string()),
                    );
                }
            }
            None => {
                return gen_std_json_response::<GetData>(None, Some("档案不存在！".to_string()));
            }
        },
        Err(err) => {
            println!("Err:{:?}", err);
            return gen_std_json_response::<GetData>(None, Some("数据库查询错误！".to_string()));
        }
    }
}

#[post("/submit")]
pub async fn submit_profile(
    app_state: web::Data<AppState>,
    form_data: web::Json<Profile>,
) -> HttpResponse {
    // TODO: uuid验证（可能的查询字符串注入？）
    let mut parsed_data = form_data.into_inner();
    let uuid = &parsed_data.uuid;
    match app_state.profile.get(doc! {"uuid":&uuid}).await {
        Ok(result) => match result {
            Some(inner) => {
                if inner.get("init").and_then(Bson::as_bool).unwrap() {
                    gen_std_json_response(
                        Some(UpdateData {
                            exist: true,
                            init: true,
                            updated_count: 0,
                        }),
                        None,
                    )
                } else {
                    if parsed_data.openID != "" {
                        //TODO: 错误处理
                        let id_response = reqwest::get(&format!("https://api.weixin.qq.com/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",appid=get_config("APPID"),secret=get_config("APPSECRET"),code=parsed_data.openID)).await.unwrap();
                        let parsed_response: OpenIDResponse =
                            serde_json::from_str(&id_response.text().await.unwrap()).unwrap();
                        match parsed_response.errcode {
                            -1 => {
                                return gen_std_json_response::<UpdateData>(
                                    None,
                                    Some("微信服务器繁忙！".to_string()),
                                );
                            }
                            40029 => {
                                return gen_std_json_response::<UpdateData>(
                                    None,
                                    Some("用户代码无效！".to_string()),
                                );
                            }
                            45011 => {
                                return gen_std_json_response::<UpdateData>(
                                    None,
                                    Some("达到频率上限！".to_string()),
                                );
                            }
                            0 => {
                                parsed_data.openID = parsed_response.openid.unwrap();
                            }
                            _ => {
                                return gen_std_json_response::<UpdateData>(
                                    None,
                                    Some("微信服务器错误！".to_string()),
                                );
                            }
                        }
                    }
                    match app_state
                        .profile
                        .set(
                            doc! {"uuid":&uuid},
                            bson::to_document(&parsed_data).unwrap(),
                        )
                        .await
                    {
                        Ok(result) => {
                            return gen_std_json_response(
                                Some(UpdateData {
                                    exist: true,
                                    init: false,
                                    updated_count: result.modified_count,
                                }),
                                None,
                            );
                        }
                        Err(err) => {
                            println!("Err:{:?}", err);
                            return gen_std_json_response::<UpdateData>(
                                None,
                                Some("数据库修改错误！".to_string()),
                            );
                        }
                    };
                }
            }
            None => gen_std_json_response(
                Some(UpdateData {
                    exist: false,
                    init: false,
                    updated_count: 0,
                }),
                None,
            ),
        },
        Err(err) => {
            println!("Err:{:?}", err);
            return gen_std_json_response::<UpdateData>(None, Some("数据库查询错误！".to_string()));
        }
    }
}

#[get("/verify")]
pub async fn verify_token(_: JWTAuthService, req: HttpRequest) -> HttpResponse {
    let secret = get_config("JWTSECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());
    let token = req.cookie("JWT").unwrap().to_string();
    let parsed_token = decode::<Claims>(&token, &key, &Validation::new(Algorithm::HS256));
    gen_std_json_response(
        Some(VerifyData {
            username: parsed_token.unwrap().claims.usr,
        }),
        None,
    )
}

#[post("/login")]
pub async fn login(user: web::Json<LoginRequest>, app_state: web::Data<AppState>) -> HttpResponse {
    let parsed_user = user.into_inner();
    match app_state
        .user
        .get(doc! {"username":&parsed_user.username})
        .await
    {
        Ok(result) => match result {
            Some(inner) => {
                let stored_user: User = bson::from_document(inner).unwrap();
                let mut mac = HmacSha256::new_varkey(&get_config("PWSALT").as_bytes()).unwrap();
                mac.update(parsed_user.pwhashed.as_bytes());
                let hash_result = mac.finalize();
                if format!("{:X}", hash_result.into_bytes()) == stored_user.pwhashed {
                    let claims = Claims {
                        usr: parsed_user.username,
                        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
                    };
                    let key = EncodingKey::from_secret(get_config("JWTSECRET").as_bytes());
                    let token = encode(&Header::default(), &claims, &key);
                    match token {
                        Err(err) => {
                            println!("{}", err);
                            return gen_std_json_response::<LoginData>(
                                None,
                                Some("生成JWT错误！".to_string()),
                            );
                        }
                        Ok(token) => {
                            let token_cookie = Cookie::build("JWT", token)
                                .domain("null")
                                //TODO: .domain("medkit.j4ger.cn")
                                //TODO: .path("/secure")
                                //TODO: .secure(true)
                                //TODO: .http_only(true)
                                //TODO: .same_site(SameSite::Lax)
                                .finish();
                            return HttpResponse::build(StatusCode::OK)
                                .cookie(token_cookie)
                                .content_type("application/json")
                                .json(STDJSONResponse::<LoginData> {
                                    success: true,
                                    data: Some(LoginData { JWT: true }),
                                    errmsg: None,
                                });
                        }
                    }
                } else {
                    return gen_std_json_response::<LoginData>(
                        None,
                        Some("密码不正确！".to_string()),
                    );
                }
            }
            None => {
                return gen_std_json_response::<LoginData>(
                    None,
                    Some("用户名不存在！".to_string()),
                );
            }
        },
        Err(err) => {
            println!("Err:{:?}", err);
            return gen_std_json_response::<LoginData>(None, Some("数据库查询错误！".to_string()));
        }
    }
}

#[post("/register")]
async fn raw_register(
    user: web::Json<LoginRequest>,
    app_state: web::Data<AppState>,
    //    _: JWTAuthService,
) -> HttpResponse {
    let parsed_user = user.into_inner();
    match app_state
        .user
        .get(doc! {"username":&parsed_user.username})
        .await
    {
        Ok(result) => match result {
            Some(_) => {
                return gen_std_json_response::<RegisterData>(
                    None,
                    Some("用户已存在！".to_string()),
                )
            }
            None => {
                let mut mac = HmacSha256::new_varkey(&get_config("PWSALT").as_bytes()).unwrap();
                mac.update(parsed_user.pwhashed.as_bytes());
                let hash_result = mac.finalize();
                match app_state.user.create(doc!{"username":&parsed_user.username,"pwhashed":format!("{:X}",hash_result.into_bytes())}).await {
                    Ok(_) =>{
                        return gen_std_json_response(Some(RegisterData{username:parsed_user.username}),None);
                    },
                    Err(err) => {
                        println!("Err:{:?}", err);
                        return gen_std_json_response::<RegisterData>(None, Some("数据库查询错误！".to_string()));
                    }
                }
            }
        },
        Err(err) => {
            println!("Err:{:?}", err);
            return gen_std_json_response::<RegisterData>(
                None,
                Some("数据库查询错误！".to_string()),
            );
        }
    }
}
