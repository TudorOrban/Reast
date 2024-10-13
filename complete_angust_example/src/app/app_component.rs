
 use std::collections::HashMap;

use angust::rendering::elements::component::{component::Component, component_factory::ComponentFactory};


pub struct AppComponent {
    component: Component<AppComponentState>,    
}

#[derive(Clone)]
pub struct AppComponentState {
    content: String,
}

impl AppComponentState {
    fn new() -> Self {
        Self { content: String::from("Hello, App Component!") }
    }
}

impl AppComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        let state_factory = || AppComponentState::new();

        registry.insert("app-component".to_string(), Box::new(move || {
            Box::new(
                Component::new(
                    "app-component".to_string(),
                    "src/app/app_component.html".to_string(),
                    state_factory() 
                )
            )
        }));
    }
}
    