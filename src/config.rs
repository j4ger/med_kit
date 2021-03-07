use envfile::EnvFile;
use std::path::Path;

pub fn get_config(key: &str) -> String {
    let env = EnvFile::new(&Path::new("src/config.env")).unwrap();
    env.get(key).unwrap().to_string()
}
