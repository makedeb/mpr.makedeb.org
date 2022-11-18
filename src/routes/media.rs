use rocket::get;
use std::path::PathBuf;
use include_dir::{include_dir, Dir};

static IMAGE_FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/media/images");
static CSS_JS_FILES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/css-js/");

#[get("/media/<path..>")]
pub async fn media_assets(path: PathBuf) -> Option<Vec<u8>> {
    if let Some(file) = CSS_JS_FILES.get_file(&path).map(|file| file.contents().to_vec()) {
        Some(file)
    } else if let Some(file) = IMAGE_FILES.get_file(&path).map(|file| file.contents().to_vec()) {
        Some(file)
    } else {
        None
    }
}