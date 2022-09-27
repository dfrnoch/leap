pub mod appimage;
pub mod cli;
pub mod dirs;

fn main() {
let release = appimage::github::fetch_release("AppImage/AppImageKit").unwrap();
print!("{:?}", release);

}
