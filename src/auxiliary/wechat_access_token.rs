use chrono::prelude::*;
use chrono::{DateTime, Duration};

use isahc::{self, AsyncReadResponseExt, ReadResponseExt};

use log::error;
use rocket::tokio::sync::RwLock;
use serde::Deserialize;

use std::env;

use crate::auxiliary::GenericError;

lazy_static! {
    pub static ref WECHAT_APPID: String = env::var("WECHAT_APPID").expect("未设置WECHAT_APPID");
    pub static ref WECHAT_APPSECRET: String =
        env::var("WECHAT_APPSECRET").expect("未设置WECHAT_APPSECRET");
}

pub struct WechatAccessToken {
    access_token: String,
    expiration_time: DateTime<Utc>,
}

pub struct WechatAccessTokenState {
    pub state: RwLock<WechatAccessToken>,
}

#[derive(Deserialize)]
struct WechatAccessTokenResponse {
    access_token: String,
    expires_in: i32,
}

impl WechatAccessTokenState {
    pub fn new() -> Result<Self, GenericError> {
        match isahc::get(format!(
            "https://api.weixin.qq.com\
            /cgi-bin/token?grant_type=client_credential&\
            appid={}&secret={}",
            *WECHAT_APPID, *WECHAT_APPSECRET
        )) {
            Ok(mut response) => match response.json::<WechatAccessTokenResponse>() {
                Ok(parsed_response) => {
                    let current_time = Utc::now();
                    let expiration_time =
                        current_time + Duration::seconds((parsed_response.expires_in - 360) as i64);
                    Ok(Self {
                        state: RwLock::new(WechatAccessToken {
                            access_token: parsed_response.access_token,
                            expiration_time,
                        }),
                    })
                }
                Err(error) => {
                    error!("获取AccessToken时出错：{:?}", error);
                    panic!()
                }
            },
            Err(error) => {
                error!("获取AccessToken时出错：{:?}", error);
                panic!()
            }
        }
    }

    async fn refetch(&self) -> Result<(), GenericError> {
        let parsed_response: WechatAccessTokenResponse = isahc::get_async(format!(
            "\
        https://api.weixin.qq.com/cgi-bin/token?\
        grant_type=client_credential&\
        appid={}&secret={}\
        ",
            *WECHAT_APPID, *WECHAT_APPSECRET
        ))
        .await
        .map_err(|error| {
            error!("获取AccessToken时出错：{:?}", error);
            GenericError::GetWechatAccessTokenError
        })?
        .json()
        .await
        .map_err(|error| {
            error!("获取AccessToken时出错：{:?}", error);
            GenericError::GetWechatAccessTokenError
        })?;
        let current_time = Utc::now();
        let expiration_time =
            current_time + Duration::seconds((parsed_response.expires_in - 360) as i64);
        let mut writer = self.state.write().await;
        writer.expiration_time = expiration_time;
        writer.access_token = parsed_response.access_token;
        Ok(())
    }

    pub async fn get(&self) -> Result<String, GenericError> {
        let current_time = Utc::now();
        let valid_time = self.state.read().await.expiration_time;
        if current_time > valid_time {
            self.refetch().await?;
        }
        Ok(self.state.read().await.access_token.to_owned())
    }
}
