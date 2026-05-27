use std::{
    env,
    fs,
    path::PathBuf,
};

pub fn load_config_file() -> String {
    let home = env::var("HOME").unwrap();

    let filepath = PathBuf::from(home).join(".config/inputd/config.binds");

    fs::read_to_string(filepath).expect("failed to read binds file")
}
