use std::env;
use std::path::PathBuf;

pub fn get_dir(var: &str, join: &str) -> PathBuf {
    let dir = env::var_os(var).map(|path| PathBuf::from(path).join(join));
    if let Some(ref dir) = dir {
        std::fs::create_dir_all(dir).unwrap();
    }
    dir.unwrap()
}

pub fn cache_dir() -> PathBuf {
    get_dir("HOME", ".cache/leap")
}

pub fn data_dir() -> PathBuf {
    get_dir("HOME", ".local/share/leap")
}
