#[cfg(windows)]
use windres::Build;

fn main() {
    #[cfg(windows)]
    Build::new().compile("tray.rc").unwrap();
}
