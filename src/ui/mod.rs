#![allow(non_snake_case)]


use dioxus::prelude::*;

mod menu;
mod props;
mod convertUpload;



pub fn App(cx: Scope) -> Element {

    let mut app_state = use_state(cx, || props::WindowTypes::ConvertUpload); 
    let mut file_obj = use_state(cx, || props::FileObject::new_empty());

    let stateHandler = move |value: props::WindowTypes| {app_state.set(value)};
    let cuHandler = move |value: String| {
        println!("Uploaded: {value:?}");
        match props::FileObject::new_file(value) {
            Some(obj) => file_obj.set(obj),
            _ => println!("Invalid File")
        }
        // file_obj.set(props::FileObject::new_file(value));
    };


    cx.render(rsx! {
        div {
            script { 
                include_str!("./web/jquery-3.7.0.min.js")
            }
            style { include_str!("./web/style.css") }
            main {
                div { id: "debug", format!("State: {0:?}, File: {1:?} ", app_state, file_obj.file_address) }
                div { class: "main-wrapper",
                    match app_state.get() {
                        props::WindowTypes::MainMenu => rsx! { menu::Menu {
                            on_state_change: stateHandler
                        }},
                        props::WindowTypes::ConvertUpload => rsx! { convertUpload::ConvertUpload {
                            on_state_change: stateHandler,
                            on_file_upload: cuHandler
                        }},
                        _ => rsx! {menu::Menu {
                            on_state_change: stateHandler
                        }}

                    }
                }
            }
        }
    })
}

