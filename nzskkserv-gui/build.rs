use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=icons/icon.ico");

    let file = std::fs::File::open("icons/icon.ico").unwrap();
    let icon_dir = ico::IconDir::read(file).unwrap();
    let image = icon_dir.entries()[0].decode().unwrap();
    let rgba = image.rgba_data();

    let out_file = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("icon.rs");
    let mut out_file = std::fs::File::create(out_file).unwrap();
    let source = format!(
        r#"
pub const ICON_DATA: &[u8] = &[{}];
pub const ICON_WIDTH: u32 = {};
pub const ICON_HEIGHT: u32 = {};
        "#,
        rgba.iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(","),
        image.width(),
        image.height()
    );
    write!(out_file, "{}", source).unwrap();

    #[cfg(windows)]
    windres::Build::new().compile("icon.rc").unwrap();
}
