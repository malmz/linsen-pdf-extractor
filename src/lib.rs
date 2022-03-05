mod parse;
mod pdf;

use chrono::{DateTime, Utc};
use serde::Serialize;
use wasm_bindgen::prelude::*;

pub use parse::parse;
pub use pdf::extract;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone, Default, Serialize)]
pub struct Dishes {
    pub swedish: Vec<String>,
    pub english: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Menu {
    pub date: DateTime<Utc>,
    pub dishes: Dishes,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct WeekMenu {
    pub days: Vec<Menu>,
}

#[wasm_bindgen]
pub fn extract_pdf(pdf_data: &[u8]) -> Result<JsValue, JsValue> {
    let text = extract(pdf_data).map_err(|e| e.to_string())?;
    let (_, menu) = parse(&text).map_err(|e| e.to_string())?;
    let js_value = serde_wasm_bindgen::to_value(&menu)?;
    Ok(js_value.into())
}

#[cfg(test)]
mod tests {

    #[test]
    fn extract_text() {
        let data = include_bytes!("../assets/meny.pdf");
        let text = super::extract(data).unwrap();
        //std::fs::write("out.txt", &text).unwrap();
        assert_eq!(text, include_str!("../assets/generated.txt"));
    }

    #[test]
    fn parse_text() {
        let data = include_str!("../assets/generated.txt");
        let (_, menu) = super::parse(data).unwrap();
        dbg!(&menu);
        assert_eq!(menu.days.len(), 5);
    }
}
