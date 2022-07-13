use oxipng::{optimize_from_memory, Options};
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compress_image(image_data: &[u8]) -> Vec<u8> {
    set_panic_hook();
    let options = Options {
        ..Options::default()
    };
    match optimize_from_memory(image_data, &options) {
        Ok(result) => result,
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_image() {
        let current_dir = std::env::current_dir().unwrap();
        println!("{:?}", current_dir);
        let file_path = current_dir.join("src/test.png");
        let file = std::fs::read(file_path).unwrap();
        let result = compress_image(&file);
        let percentage = (result.len() as f32 / file.len() as f32) * 100.0;
        println!("result size {}%", percentage);
    }
}
