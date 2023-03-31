use yew::prelude::*;
use log::info;
mod name_entry;

use crate::name_entry::NameEntry;

#[function_component(App)]
fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());
    let on_name_submit = Callback::from(|message: String| {
       info!("Hello {}", message); 
    });

    html! {
        <NameEntry on_submit={on_name_submit} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

