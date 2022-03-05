use std::{collections::BTreeMap, mem};

use anyhow::Result;
use lopdf::Object;

#[derive(Debug, Clone)]
struct TextObject {
    text: String,
    pos: (f64, f64),
}

pub fn extract(data: &[u8]) -> Result<String> {
    let pdf = lopdf::Document::load_mem(data)?;
    let mut lines = BTreeMap::new();
    for page in pdf.page_iter() {
        let fonts = pdf.get_page_fonts(page);
        let encodings = fonts
            .into_iter()
            .map(|(name, font)| (name, font.get_font_encoding()))
            .collect::<BTreeMap<_, _>>();
        let content = pdf.get_and_decode_page_content(page)?;

        let mut current_text = String::new();
        let mut current_encoding = None;
        let mut current_coords = None;
        for operation in &content.operations {
            match (operation.operator.as_ref(), operation.operands.as_slice()) {
                ("BT", _) => {
                    current_coords = None;
                    //current_encoding = None;
                    if !current_text.is_empty() {
                        current_text = String::new();
                    }
                }
                ("Tf", [Object::Name(name), _]) => {
                    current_encoding = encodings.get(name).cloned();
                }
                ("Td", [Object::Real(x), Object::Real(y)]) => {
                    current_coords = Some((x, y));
                }
                ("Tm", [_, _, _, _, Object::Real(x), Object::Real(y)]) => {
                    current_coords = Some((x, y));
                }
                ("Tj", [Object::String(text, _)]) => {
                    let decoded_text = lopdf::Document::decode_text(current_encoding, text);
                    current_text.push_str(&decoded_text);
                }
                ("TJ", [Object::Array(arr)]) => {
                    for op in arr {
                        match op {
                            Object::String(text, _) => {
                                let decoded_text =
                                    lopdf::Document::decode_text(current_encoding, text);
                                current_text.push_str(&decoded_text);
                            }
                            _ => {}
                        }
                    }
                }
                ("ET", _) => {
                    if let Some((x, y)) = current_coords.take() {
                        if current_text.is_empty() {
                            continue;
                        }

                        let text = mem::take(&mut current_text);

                        lines
                            .entry(*y as u32)
                            .or_insert_with(Vec::new)
                            .push(TextObject {
                                text,
                                pos: (*x, *y),
                            });
                    }
                }
                _ => {}
            }
        }
    }

    let mut text = String::new();

    for line in lines.values().rev() {
        let mut line = line.clone();
        line.sort_by_key(|t| t.pos.0 as u32);

        for t in line {
            /* if t.text.trim().is_empty() {
                continue;
            } */
            text.push_str(&t.text);
        }
        text.push('\n');
    }
    Ok(text)
}
