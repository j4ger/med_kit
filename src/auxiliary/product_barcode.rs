use chrono::prelude::*;

use rocket::{request::FromParam, tokio::sync::RwLock};

use serde::{Deserialize, Serialize};
use serde_json;

use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
};
pub struct ProductBarcode<'a>(&'a str);

impl<'a> FromParam<'a> for ProductBarcode<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        if param.chars().all(|c| c.is_numeric()) {
            Ok(ProductBarcode(param))
        } else {
            Err(param)
        }
    }
}

impl<'a> ProductBarcode<'a> {
    pub fn inner(self) -> &'a str {
        self.0
    }
}

#[derive(Deserialize, Serialize)]
pub struct ProductBarcodeGenerator {
    last_datetime: DateTime<Utc>,
    next_index: i32,
}

pub struct ProductBarcodeGeneratorState {
    state: RwLock<ProductBarcodeGenerator>,
}

impl ProductBarcodeGeneratorState {
    pub fn new() -> Self {
        let current_datetime = Utc::now();
        Self {
            state: RwLock::new(ProductBarcodeGenerator {
                last_datetime: current_datetime,
                next_index: 0,
            }),
        }
    }

    pub async fn get(&self) -> String {
        let current_datetime = Utc::now();
        let last_day = self.state.read().await.last_datetime.day();
        let mut writer = self.state.write().await;
        if current_datetime.day() != last_day {
            writer.last_datetime = current_datetime;
            writer.next_index = 0;
        }
        let index = writer.next_index;
        writer.next_index += 1;
        format!("{}{:08}", current_datetime.format("%y%m%d"), index)
    }

    pub async fn save(&self) {
        let mut full_path = env::current_dir().expect("工作路径获取失败");
        let relative_path = env::var("STATE_FILE").expect("未设置STATE_FILE");
        full_path.push(relative_path);
        let mut state_file = File::create(full_path).expect("创建STATE_FILE失败");
        let current_state = &*self.state.read().await;
        let content = serde_json::to_string(current_state).expect("编码STATE失败");
        state_file
            .write_all(content.as_bytes())
            .expect("STATE写入失败")
    }

    pub fn load() -> Self {
        let mut full_path = env::current_dir().expect("工作路径获取失败");
        let relative_path = env::var("STATE_FILE").expect("未设置STATE_FILE");
        full_path.push(relative_path);
        match File::open(full_path) {
            Ok(state_file) => match serde_json::from_reader(BufReader::new(state_file)) {
                Ok(generator) => Self {
                    state: RwLock::new(generator),
                },
                Err(_) => Self::new(),
            },
            Err(_) => Self::new(),
        }
    }
}

impl Drop for ProductBarcodeGenerator {
    fn drop(&mut self) {
        let mut full_path = env::current_dir().expect("工作路径获取失败");
        let relative_path = env::var("STATE_FILE").expect("未设置STATE_FILE");
        full_path.push(relative_path);
        let mut state_file = File::create(full_path).expect("创建STATE_FILE失败");
        let content = serde_json::to_string(self).expect("编码STATE失败");
        state_file
            .write_all(content.as_bytes())
            .expect("STATE写入失败")
    }
}
