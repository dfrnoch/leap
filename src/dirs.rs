use std::env;
use std::path::PathBuf;

pub fn get_dir(var: &str, join: String) -> PathBuf {
    let dir = env::var_os(var).map(|path| PathBuf::from(path).join(join));
    if let Some(ref dir) = dir {
        std::fs::create_dir_all(dir).unwrap();
    }
    dir.unwrap()
}

pub fn cache_dir() -> PathBuf {
    get_dir("HOME", ".cache/leap".to_owned())
}

pub fn bin_dir() -> PathBuf {
    get_dir("HOME", ".local/bin".to_owned())
}

pub fn data_dir(p: Option<&str>) -> PathBuf {
    get_dir("HOME", ".local/share/leap/".to_owned() + p.unwrap_or(""))
}
