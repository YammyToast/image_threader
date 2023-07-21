use dioxus::prelude::*;

use super::props;

#[inline_props]
pub fn ConfigureConvert<'a>(
    cx: Scope<'a>,
    on_state_change: EventHandler<'a, props::WindowTypes>,
    file_obj: &'a props::FileObject,
    on_output_dimension_change: EventHandler<'a, (Option<u32>, Option<u32>)>
    
) -> Element<'a> {
    if (cx.props.file_obj.loaded == false) {
        cx.props
            .on_state_change
            .call(props::WindowTypes::ConvertUpload)
    }

    cx.render(rsx!(
        script {
            include_str!("./web/configure.js")
        }
        input {
            class: "text-invis",
            id: "render_data_buffer",
            r#type: "text",
            name: "render_default_buffer",
            value: "{cx.props.file_obj.data_url}"
        },
        div { class: "cu-hidden-image-render", id: "hidden-image-default"

        },
        div { class: "cc-row-grid",
            div { class: "cc-title",
                "Converting \"", cx.props.file_obj.file_address.clone(), "\""
            },
            div { class: "cc-content-row-grid",
                div { class: "cc-canvas", id: "cc-default-canvas"}
                div {
                    div { class: "cc-input-item",
                        div { class: "cc-input-label",
                            "Image Output Dimensions:"
                        },
                        div {
                            label {
                                r#for: "output-dimension-x",
                                "Width: "
                            }
                            input { class: "cc-input-text cc-input-item-small-text",
                                r#type: "text",
                                name: "output-dimension-x",
                                min: 1,
                                maxlength: 4,
                                value: "{cx.props.file_obj.width.to_string()}",
                                oninput: move |event| {

                                    cx.props.on_output_dimension_change.call((Some(event.value.clone().parse::<u32>().unwrap()), None))
                                }
                                // value: 512
                            }label {
                                r#for: "output-dimension-y",
                                "Height: "
                            }
                            input { class: "cc-input-text cc-input-item-small-text",
                                r#type: "text",
                                name: "output-dimension-y",
                                min: 1,
                                maxlength: 4,
                                value: "{cx.props.file_obj.height.to_string()}",
                                oninput: move |event| {
                                    cx.props.on_output_dimension_change.call((None, Some(event.value.clone().parse::<u32>().unwrap())))
                                }
                                // value: 512
                            }
                        }
                    },
                    div {
                        div { class: "cc-input-item",
                            div { class: "cc-input-label",
                                "Image Output Dimensions:"
                            },
                            div {
    
                            }
                        }
    
                    }

                },

            },
            div { class: "cu-footer",
            button { class: "cu-footer-button button-exit", onclick: move |event| {
                cx.props.on_state_change.call(props::WindowTypes::ConvertUpload)
                },
                "Back"
            },
            button { class: "cu-footer-button button-submit", onclick: move |event| {
                cx.props.on_state_change.call(props::WindowTypes::ConvertRender)
                },
                "Next"
            }
            }
        }
    ))
}
