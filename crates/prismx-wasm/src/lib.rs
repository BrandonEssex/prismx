use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_function() -> String {
    "PrismX WASM ready".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(test_function(), "PrismX WASM ready");
    }
}
