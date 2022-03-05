use std::slice;

#[no_mangle]
pub extern "C" fn extract_pdf(data: *const u8, len: usize) -> *const u8 {
    let data = unsafe { slice::from_raw_parts(data, len) };
    let menu = super::extract_menu_from_pdf(data);
    let json = serde_json::to_string(&menu).unwrap_or_default();
    let len = json.len() as u32;
    let mut data = len.to_be_bytes().to_vec();
    data.extend_from_slice(json.as_bytes());
    let ptr = data.as_ptr();
    std::mem::forget(data);
    ptr
}
