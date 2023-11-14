mod components;
mod enums;
mod services;
use components::{list::List, tab::Tab, tab_switcher::TabSwitcher};
use enums::selection::Selection;
use services::gitlab::{GitLab, MergeRequest};
use std::future::Future;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::Document;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn get_query_params() -> Result<(String, String), JsValue> {
    let window = web_sys::window().expect("Failed to get window object");
    let location = window.location();
    let url = location.href().expect("Failed to get URL");
    let splitted_url = url.split('?').collect::<Vec<&str>>();
    if splitted_url.len() < 2 {
        return Err(JsValue::from_str("no search query found in url"));
    }
    let search = splitted_url[1];
    let mut access_token = None;
    let mut host = None;
    let param_values = search.split('&').collect::<Vec<&str>>();
    param_values.iter().for_each(|param_value| {
        if param_value.contains("access_token") {
            access_token = param_value.split('=').collect::<Vec<&str>>().pop();
        }
        if param_value.contains("host") {
            host = param_value.split('=').collect::<Vec<&str>>().pop();
        }
    });
    if host.is_some() && access_token.is_some() {
        Ok((host.unwrap().to_owned(), access_token.unwrap().to_owned()))
    } else {
        Err(JsValue::from_str(
            "Error:  access_token and host must both be defined in url as query params",
        ))
    }
}

async fn build_tab(
    selection: Selection,
    service: impl Future<Output = Result<Vec<MergeRequest>, JsValue>>,
    tab_switcher: &Rc<TabSwitcher>,
    active: bool,
) -> Result<(), JsValue> {
    let fetched_mrs = service.await?;
    let mut list = List::new();
    list.set_items(fetched_mrs);
    let mut tab = Tab::new();
    tab.set_value(selection);
    tab.set_active(active);
    tab.set_content(list.node);
    tab_switcher.append_tab(tab);
    Ok(())
}

// Called by our JS entry point
#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    let (host, access_token) = get_query_params()?;

    console_log!("{:?} {:?}", host, access_token);

    let service_instance = GitLab::new(
        host,
        access_token,
    )
    .await?;

    let window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let general_container = document.create_element("div")?;
    general_container.set_attribute("class", "general-container")?;

    let tab_switcher = TabSwitcher::new();

    build_tab(
        Selection::ToReview,
        service_instance.get_mr_to_review(),
        &tab_switcher,
        true,
    )
    .await?;
    build_tab(
        Selection::UnderReview,
        service_instance.get_mr_under_review(),
        &tab_switcher,
        false,
    )
    .await?;
    build_tab(
        Selection::Draft,
        service_instance.get_mr_draft(),
        &tab_switcher,
        false,
    )
    .await?;

    general_container.append_child(&Rc::clone(&tab_switcher).container_node)?;
    body.append_child(&general_container)?;

    Ok(())
}
