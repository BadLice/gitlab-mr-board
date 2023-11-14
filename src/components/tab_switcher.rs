use crate::components::tab::Tab;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{window, Element, Event, HtmlElement};

pub struct TabSwitcher {
    pub switcher_node: Element,
    pub container_node: Element,
    pub content_node: Element,
    pub tabs: RefCell<Vec<Tab>>,
}

impl TabSwitcher {
    pub fn new() -> Rc<Self> {
        let document = window().unwrap().document().unwrap();

        let container_node = document.create_element("div").unwrap();
        container_node
            .set_attribute("class", "tab-switcher-container")
            .unwrap();

        let switcher_node = document.create_element("div").unwrap();
        switcher_node
            .set_attribute("class", "tab-switcher")
            .unwrap();
        switcher_node.set_attribute("name", "tab-switcher").unwrap();

        let content_node = document.create_element("div").unwrap();
        content_node.set_attribute("class", "tab-switcher-content").unwrap();

        container_node.append_child(&switcher_node).unwrap();
        container_node.append_child(&content_node).unwrap();

        Rc::new(Self {
            container_node,
            switcher_node,
            content_node,
            tabs: RefCell::new(Vec::new()),
        })
    }
    pub fn append_tab(self: &Rc<Self>, mut tab: Tab) {
        let leaked_tab_switcher = Box::leak(Box::new(Rc::clone(self)));

        let change_selection = move |e: Event| {
            let selected_value = e
                .current_target()
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .get_attribute("value")
                .unwrap();

            leaked_tab_switcher
                .tabs
                .borrow_mut()
                .iter_mut()
                .for_each(|tab| {
                    if let Some(value) = &tab.value {
                        if value.to_string() == selected_value {
                            tab.set_active(true);
                        } else {
                            tab.set_active(false);
                        }
                    }
                });
        };
        tab.set_on_click(Closure::new(change_selection));
        self.switcher_node.append_child(&tab.action_node).unwrap();
        self.content_node.append_child(&tab.content_node).unwrap();
        self.tabs.borrow_mut().push(tab);
    }
}
