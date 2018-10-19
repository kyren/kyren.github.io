use failure::{err_msg, Error};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, HtmlElement};

pub fn console_log<S: Into<JsValue>>(s: S) {
    console::log_1(&s.into())
}

pub fn get_element<T: JsCast>(id: &str) -> Result<T, Error> {
    web_sys::window()
        .ok_or_else(|| err_msg("no window"))?
        .document()
        .ok_or_else(|| err_msg("no document"))?
        .get_element_by_id(id)
        .ok_or_else(|| err_msg(format!("no such element with id '{}'", id)))?
        .dyn_into::<T>()
        .map_err(|_| err_msg(format!("element '{}' incorrect type", id)))
}

pub fn handle_error<F: FnOnce() -> Result<(), Error>>(context: &str, f: F) {
    if let Err(err) = f() {
        console_log(format!("{} errored with: {:?}", context, err));
        panic!();
    }
}

pub fn js_err(e: JsValue) -> Error {
    err_msg(e.as_string().unwrap_or("unknown error".to_owned()))
}

pub fn show_element(element: &HtmlElement) -> Result<(), Error> {
    element
        .style()
        .set_property("display", "block")
        .map_err(|e| {
            err_msg(
                e.as_string()
                    .unwrap_or("unknown error setting element property".to_owned()),
            )
        })
}

pub fn hide_element(element: &HtmlElement) -> Result<(), Error> {
    element
        .style()
        .set_property("display", "none")
        .map_err(|e| {
            err_msg(
                e.as_string()
                    .unwrap_or("unknown error setting element property".to_owned()),
            )
        })
}
