
extern crate angust;

use angust::application::application::Application;

pub mod app;
pub mod component_registration;


pub struct AppGlobalState {
    pub message: String,
}

fn main() {
    let initial_state = AppGlobalState {
        message: "Hello, Angust user!".to_string(),
    };

    component_registration::register_components();    

    let mut app = Application::new(initial_state, String::from("New Angust App"));
    
    app.run();
}
    
    