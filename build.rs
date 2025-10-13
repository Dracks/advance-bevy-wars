use assets_generator::build_assets_enum;
use std::path::Path;

fn main() {
    build_assets_enum(&Path::new("src").join("assets.rs"), None, None);
}
