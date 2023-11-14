use crate::components::helpers::class_helpers::{append_class_name, remove_class_name};
use crate::enums::selection::Selection;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Element, Event};

pub struct Tab {
    pub action_node: Element,
    pub content_node: Element,
    pub value: Option<Selection>,
}

impl Tab {
    pub fn new() -> Self {
        let document = window().unwrap().document().unwrap();
        let action_node = document.create_element("div").unwrap();
        action_node.set_attribute("class", "tab-action").unwrap();
        action_node.set_attribute("name", "tab-action").unwrap();

                let content_node = document.create_element("div").unwrap();
        content_node.set_attribute("class", "tab-content").unwrap();

        Self { value: None, action_node, content_node }
    }
    pub fn set_active(&mut self, active: bool) {
        if active {
            append_class_name(&self.action_node, "active");
            append_class_name(&self.content_node, "active");
        } else {
            remove_class_name(&self.action_node, "active");
            remove_class_name(&self.content_node, "active");
        }
    }
    pub fn set_value(&mut self, value: Selection) {
        self.value = Some(value);
        let text_value = value.to_string();
        self.action_node.set_attribute("value", &text_value).unwrap();

        self.action_node.set_text_content(Some(&text_value));
    }
    pub fn set_on_click(&mut self, on_click: Closure<dyn FnMut(Event)>) {
        let leaked_event_handler = Box::leak(Box::new(on_click));
        self.action_node
            .add_event_listener_with_callback(
                "click",
                leaked_event_handler.as_ref().unchecked_ref(),
            )
            .unwrap();
    }
    pub fn set_content(&mut self, node: Element) {
        self.content_node.append_child(&node).unwrap();
    }
}
