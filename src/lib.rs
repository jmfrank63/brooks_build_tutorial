use gloo::console::log;
use serde::Serialize;
use serde_json::to_string_pretty;
use yew::prelude::*;

#[derive(Debug, Serialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "Johannes";
    let my_object = MyObject {
        username: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };
    log!("My name is: ", name);
    log!(format!("My object: {:?}", my_object));
    log!("My object", to_string_pretty(&my_object).unwrap());
    let class = "my_title";
    html! {
        <>
        <h1 class = {class}>{"Hello World!!!"}</h1>
        <p>{format!("My name is {}", name)}</p>
        </>
    }
}
