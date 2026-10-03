use crate::models::Item;
use wasm_bindgen::JsValue;

const STORAGE_KEY: &str = "find_my_stuff_items";

fn local_storage() -> Result<web_sys::Storage, JsValue> {
    web_sys::window()
        .ok_or_else(|| JsValue::from_str("Window unavailable"))?
        .local_storage()?
        .ok_or_else(|| JsValue::from_str("Local storage unavailable"))
}

pub fn load_items() -> Result<Vec<Item>, JsValue> {
    let storage = local_storage()?;

    let Some(data) = storage.get_item(STORAGE_KEY)? else {
        return Ok(Vec::new());
    };

    serde_json::from_str(&data)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

pub fn save_items(items: &[Item]) -> Result<(), JsValue> {
    let storage = local_storage()?;

    let data = serde_json::to_string(items)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    storage.set_item(STORAGE_KEY, &data)?;

    Ok(())
}

pub fn add_item(item: Item) -> Result<Vec<Item>, JsValue> {
    let mut items = load_items()?;

    items.push(item);

    save_items(&items)?;

    Ok(items)
}

pub fn update_item(updated: Item) -> Result<Vec<Item>, JsValue> {
    let mut items = load_items()?;

    if let Some(item) = items.iter_mut().find(|item| item.id == updated.id) {
        *item = updated;
    }

    save_items(&items)?;

    Ok(items)
}

pub fn delete_item(id: u32) -> Result<Vec<Item>, JsValue> {
    let mut items = load_items()?;

    items.retain(|item| item.id != id);

    save_items(&items)?;

    Ok(items)
}

pub fn next_id() -> Result<u32, JsValue> {
    let items = load_items()?;

    Ok(items
        .iter()
        .map(|item| item.id)
        .max()
        .unwrap_or(0)
        + 1)
}




