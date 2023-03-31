use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NameEntryProps {
    pub on_submit: Callback<String>
}

#[function_component(NameEntry)]
pub fn name_entry(NameEntryProps { on_submit }: &NameEntryProps) -> Html {
    // Html form with onSubmit invoking the callback
    let on_submit = on_submit.clone();
    let on_click = move |_: MouseEvent| {
       on_submit.emit(String::from("Hello world")); 
    };

    html! {
        <button onclick={on_click}>{ "Hello World" }</button>
    }
}

