pub mod ffi;
mod parse;
mod pdf;

pub use parse::parse;
use parse::WeekMenu;
pub use pdf::extract;

pub fn extract_menu_from_pdf(pdf_data: &[u8]) -> WeekMenu {
    match extract(pdf_data) {
        Ok(text) => parse(&text).map(|v| v.1).unwrap_or_default().into(),
        Err(_) => Default::default(),
    }
}

/* #[deno_bindgen::deno_bindgen]
struct Boi {
    data: String,
}

#[deno_bindgen::deno_bindgen]
fn testing() -> Boi {
    Boi {
        data: "Hello".to_string(),
    }
} */

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
