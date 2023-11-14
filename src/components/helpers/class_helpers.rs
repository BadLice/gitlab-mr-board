use web_sys::Element;

pub fn append_class_name(element: &Element, class_name: &str) {
    let current_class_name = element.class_name();
    if !current_class_name.contains(class_name) {
        let new_class_name = format!("{} {}", current_class_name, class_name);
        element.set_class_name(&new_class_name);
    }
}

pub fn remove_class_name(element: &Element, class_name: &str) {
    let current_class_name = element.class_name();
    if current_class_name.contains(class_name) {
        let new_class_name = current_class_name.replace(class_name, "");
        element.set_class_name(&new_class_name);
    }
}

pub fn toggle_class_name(element: &Element, class_name: &str) {
    let current_class_name = element.class_name();
    if current_class_name.contains(class_name) {
        let new_class_name = current_class_name.replace(class_name, "");
        element.set_class_name(&new_class_name);
    } else {
        let new_class_name = format!("{} {}", current_class_name, class_name);
        element.set_class_name(&new_class_name);
    }
}
