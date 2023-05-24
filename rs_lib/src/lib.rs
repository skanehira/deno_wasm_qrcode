use wasm_bindgen::prelude::*;
use qrcode_generator::QrCodeEcc;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[wasm_bindgen]
pub fn generate(text: String) -> Vec<u8> {
  let result: Vec<u8> =
    qrcode_generator::to_png_to_vec(text, QrCodeEcc::Low, 512)
      .expect("cannot generate QR code");
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(1, 2);
    assert_eq!(result, 3);
  }
}
