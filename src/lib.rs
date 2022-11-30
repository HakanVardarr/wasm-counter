use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element, HtmlElement, Window};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no-global 'window' exists");
    let document = window.document().expect("should have a document on window");
    let num_clicks = document.get_element_by_id("num").unwrap();
    let value = get_clicks(&window);

    num_clicks.set_inner_html(&value.to_string());

    setup_clicker(&document, num_clicks.clone(), window.clone());

    log(&value.to_string());

    Ok(())
}

fn setup_clicker(document: &Document, num_clicks: Element, window: Window) {
    let mut clicks = num_clicks
        .clone()
        .text_content()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let a = Closure::<dyn FnMut()>::new(move || {
        clicks += 1;
        num_clicks.set_inner_html(&clicks.to_string());
        window
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item("num", &clicks.to_string())
            .unwrap();
    });

    document
        .get_element_by_id("increament-button")
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(a.as_ref().unchecked_ref()));

    a.forget();
}

fn get_clicks(window: &Window) -> i32 {
    let clicks = window.local_storage().unwrap().unwrap().get("num").unwrap();
    if let Some(num) = clicks {
        return num.parse::<i32>().unwrap();
    } else {
        #[allow(unused)]
        let popo = window
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item("num", "0");
        return 0;
    }
}
