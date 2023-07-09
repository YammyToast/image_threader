use dioxus::prelude::*;

use super::props;

#[inline_props]
pub fn ConvertUpload<'a>(cx: Scope<'a>, on_state_change: EventHandler<'a, props::WindowTypes>, on_file_upload: EventHandler<'a, (String, String)>) -> Element<'a> {
    cx.render(rsx!(
        script { 
            include_str!("./web/upload.js")
        }
        div { class: "cu-row-grid",
            div { class: "cu-title",
                "Upload an Image"},
            div {
                div { class: "cu-p",
                    "Select a file to upload. Supported image types are: "
                    br {}
                    ".png"
                    br {}
                    ".jpg"


                }
            },
            div {
                label { class: "cu-upload-card", r#for:"upload_default",
                    div { class: "cu-upload-button",
                        img {src: "./src/assets/cu-upload.svg"}
                        "Click to Upload"
                    }
                    div {
                        "TWATW"
                    }

                }
                input { 
                    id: "upload_default",
                    r#type: "file",
                    accept: ".png,.jpg",
                    name: "files[]",
                    // oninput: move |event| {
                    //     println!("{event:?}");
                    //     cx.props.on_file_upload.call((event.value.clone(), String::from("temp")))
                        
                    // }
                }
                input {
                    id: "upload_data_buffer",
                    r#type: "text",
                    name: "upload_default_buffer",
                    oninput: move |event| {
                        println!("{event:?}");
                        cx.props.on_file_upload.call((String::from("temp"), String::from("temp")));
                    }
                }
            },
            div { class: "cu-footer",
                button { class: "cu-footer-button button-exit", onclick: move |event| {
                    cx.props.on_state_change.call(props::WindowTypes::MainMenu)
                    },
                    "Back"
                }
                button { class: "cu-footer-button button-submit", onclick: move |event| {
                    cx.props.on_state_change.call(props::WindowTypes::ConfigureConvert)
                    },
                    "Convert"
                }
            },

        }
        
    ))
}
