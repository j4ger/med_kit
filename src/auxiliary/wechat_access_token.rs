use chrono::prelude::*;
use chrono::{DateTime, Duration};

use isahc::{self, get, AsyncReadResponseExt};

use isahc::ReadResponseExt;
use log::error;
use serde::{Deserialize, Serialize};

use std::env;

use crate::auxiliary::GenericError;

lazy_static! {
    pub static ref WECHAT_APPID: String = env::var("WECHAT_APPID").expect("未设置WECHAT_APPID");
    pub static ref WECHAT_APPSECRET: String =
        env::var("WECHAT_APPSECRET").expect("未设置WECHAT_APPSECRET");
}

pub struct WechatAccessToken {
    pub access_token: String,
    expiration_time: DateTime<Utc>,
}

#[derive(Deserialize)]
struct WechatAccessTokenResponse {
    access_token: String,
    expires_in: i32,
}

impl WechatAccessToken {
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
                        access_token: parsed_response.access_token,
                        expiration_time,
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

    async fn refetch(&mut self) -> Result<(), GenericError> {
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
        self.expiration_time = expiration_time;
        self.access_token = parsed_response.access_token;
        Ok(())
    }

    pub async fn get(&mut self) -> Result<String, GenericError> {
        let current_time = Utc::now();
        if current_time > self.expiration_time {
            self.refetch().await?;
        }
        Ok(self.access_token.to_owned())
    }
}
