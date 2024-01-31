use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub struct Tada {
    pub tudu: u32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
