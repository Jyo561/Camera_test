use js_sys::{Function, Object, Reflect, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub async fn take_photo() -> Result<String, JsValue> {
    let window = web_sys::window()
        .ok_or_else(|| JsValue::from_str("Window unavailable"))?;

    // window.Capacitor
    let capacitor = Reflect::get(
        &window,
        &JsValue::from_str("Capacitor"),
    )?;

    // Capacitor.Plugins
    let plugins = Reflect::get(
        &capacitor,
        &JsValue::from_str("Plugins"),
    )?;

    // Capacitor.Plugins.Camera
    let camera = Reflect::get(
        &plugins,
        &JsValue::from_str("Camera"),
    )?;

    // Camera.getPhoto
    let get_photo = Reflect::get(
        &camera,
        &JsValue::from_str("getPhoto"),
    )?;

    let get_photo: Function = get_photo
        .dyn_into()
        .map_err(|_| {
            JsValue::from_str(
                "Camera.getPhoto is not a function"
            )
        })?;

    let options = Object::new();

    Reflect::set(
        &options,
        &JsValue::from_str("quality"),
        &JsValue::from_f64(90.0),
    )?;

    Reflect::set(
        &options,
        &JsValue::from_str("allowEditing"),
        &JsValue::FALSE,
    )?;

    Reflect::set(
        &options,
        &JsValue::from_str("resultType"),
        &JsValue::from_str("dataUrl"),
    )?;

    Reflect::set(
        &options,
        &JsValue::from_str("source"),
        &JsValue::from_str("camera"),
    )?;

    // Call Camera.getPhoto(...)
    let promise = get_photo.call1(
        &camera,
        &options,
    )?;

    // Convert JS Promise → Rust Future
    let promise = Promise::from(promise);

    let result = JsFuture::from(promise).await?;

    // result.dataUrl
    let data_url = Reflect::get(
        &result,
        &JsValue::from_str("dataUrl"),
    )?;

    data_url
        .as_string()
        .ok_or_else(|| {
            JsValue::from_str(
                "Camera returned no dataUrl"
            )
        })
}
