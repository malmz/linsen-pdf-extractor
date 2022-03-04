pub mod ffi;
mod parse;
mod pdf;

pub use parse::parse;
pub use pdf::extract;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Menu {
    pub date: String,
    pub dishes: Vec<String>,
}

impl From<parse::Menu> for Menu {
    fn from(menu: parse::Menu) -> Self {
        Self {
            date: menu.date.to_rfc3339(),
            dishes: menu.dishes,
        }
    }
}

#[derive(Default, Serialize)]
pub struct WeekMenu {
    pub days: Vec<Menu>,
}

impl From<parse::WeekMenu> for WeekMenu {
    fn from(week_menu: parse::WeekMenu) -> Self {
        Self {
            days: week_menu.days.into_iter().map(|menu| menu.into()).collect(),
        }
    }
}

pub fn extract_menu_from_pdf(pdf_data: &[u8]) -> WeekMenu {
    match extract(pdf_data) {
        Ok(text) => parse(&text).map(|v| v.1).unwrap_or_default().into(),
        Err(_) => Default::default(),
    }
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
