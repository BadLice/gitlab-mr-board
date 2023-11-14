use crate::services::gitlab::MergeRequest;
use web_sys::{window, Element};

pub struct List {
    items: Vec<MergeRequest>,
    pub node: Element,
}

impl List {
    pub fn new() -> Self {
        let document = window().unwrap().document().unwrap();
        let container = document.create_element("div").unwrap();
        container.set_attribute("class", "list-container").unwrap();

        Self {
            node: container,
            items: Vec::new(),
        }
    }
    pub fn push_item(&mut self, item: MergeRequest) {
        let document = window().unwrap().document().unwrap();
        let item_node = document.create_element("div").unwrap();
        item_node.set_attribute("class", "list-item").unwrap();
        item_node.set_text_content(Some(&item.title));
        self.node.append_child(&item_node).unwrap();
        self.items.push(item);
    }
    pub fn set_items(&mut self, mut items: Vec<MergeRequest>) {
        items.drain(..).for_each(|item| self.push_item(item));
    }
}
