use wasm_bindgen::prelude::*;
use web_sys::Element;

enum Section {
    ToReview,
    UnderReview,
    Draft
}

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let mut section = Section::ToReview;

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let container = document.create_element("div")?;
    container.set_attribute("style", "background-color: red; display: flex; width: 100%; height: 100%; justify-content: center; align-items: center; flex:1")?;

    let tab_switcher = document.create_element("div")?;
    tab_switcher.set_attribute("style", "display: flex; width: 100%; justify-content: center; align-items: center; flex:1; flex-direction: row;")?;

    let tabs = [document.create_element("div")?,document.create_element("div")?,document.create_element("div")?];

    let tab_to_review = document.create_element("div")?;
    container.set_text_content(Some("To Review"));

    tabs.map(|tab| tab.set_attribute("style", "display: flex; width: 30%; justify-content: center; align-items: center; flex:1; flex-direction: row;"));


tab_switcher.append_child(&tab_to_review)?;
    
     tabs.map(|tab| container.append_child(&tab))
    ;
    body.append_child(&container)?;

    Ok(())
}
