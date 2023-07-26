#![allow(non_snake_case)]


use dioxus::prelude::*;

use crate::ui::props::FileObject;


mod menu;
mod props;
mod convertUpload;
mod configureConvert;


pub fn App(cx: Scope) -> Element {

    let app_state = use_state(cx, || props::WindowTypes::ConvertUpload); 
    let file_obj = use_state(cx, || props::FileObject::new_empty());

    let stateHandler = move |value: props::WindowTypes| {app_state.set(value)};
    let cuHandler = move |data: String| {
        let parts = data.split("?,");
        let collection = parts.collect::<Vec<&str>>();
        if (collection.len() < 2) {
            panic!("Invalid File");
        }
        match props::FileObject::new_from_url(
            String::from(collection[0]),
            String::from(collection[3]),
            String::from(collection[1]).trim().parse::<u32>().unwrap(),
            String::from(collection[2]).trim().parse::<u32>().unwrap()
            
        ) {
            Some(obj) => file_obj.set(obj),
            _ => panic!("Invalid File")
        }
    };

    let dimensionHandler = move |values: (Option<u32>, Option<u32>) | {
        match values.0 {
            None => {},            
            Some(e) => {
                file_obj.with_mut(|obj| {
                    obj.output_width = e;
                });
            }
        }
        match values.1 {
            None => {},
            Some(e) => {
                file_obj.with_mut(|obj| {
                    obj.output_height = e
                });
            }
        }
    };

    let flipHandler = move |values: (Option<bool>, Option<bool>) | {
        match values.0 {
            None => {},
            Some(e) => {
                file_obj.with_mut(|obj| {
                    obj.flip_x = e
                })
            }

        }
        match values.1 {
            None => {},
            Some(e) => {
                file_obj.with_mut(|obj| {
                    obj.flip_y = e
                })
            }

        }
    };

    println!("X: {0:?}, Y: {1:?}", file_obj.get().flip_x, file_obj.get().flip_y);

    cx.render(rsx! {
        div {
            script { 
                include_str!("./web/jquery-3.7.0.min.js")
            }
            script { 
                include_str!("./web/p5.min.js")
            }
            style { include_str!("./web/style.css") }
            main {

                // div { id: "debug", format!("State: {0:?}, File: {1:?} ", app_state, file_obj.file_address) }

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
                        props::WindowTypes::ConfigureConvert => rsx! { configureConvert::ConfigureConvert {
                            on_state_change: stateHandler,
                            file_obj: file_obj.get(),
                            on_output_dimension_change: dimensionHandler,
                            on_output_flip_change: flipHandler
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

