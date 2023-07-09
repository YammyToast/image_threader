#![allow(non_snake_case)]


use dioxus::prelude::*;

mod menu;
mod props;
mod convertUpload;



pub fn App(cx: Scope) -> Element {

    let mut app_state = use_state(cx, || props::WindowTypes::ConvertUpload); 
    let mut file_obj = use_state(cx, || props::FileObject::new_empty());

    let stateHandler = move |value: props::WindowTypes| {app_state.set(value)};
    let cuHandler = move |data: String| {
        let parts = data.split("?,");
        let collection = parts.collect::<Vec<&str>>();
        if (collection.len() < 2) {
            panic!("Invalid File");
        }

        match props::FileObject::new_from_base64(String::from(collection[0]), String::from(collection[1])) {
            Some(obj) => file_obj.set(obj),
            _ => panic!("Invalid File")
        }
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
                            on_file_upload: cuHandler,
                            file_obj: file_obj.get()
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

