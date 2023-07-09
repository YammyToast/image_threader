use dioxus::prelude::*;
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};


#[derive(Debug)]
pub enum WindowTypes {
    MainMenu,
    ConvertUpload,
    ViewUpload,
    ConfigureConvert,
    Loading,
    ConvertRender,
}

#[derive(Debug)]
pub enum ExtensionTypes {
    PNG,
    JPG,
    NA,
}

pub struct FileObject {
    pub loaded: bool,
    pub file_address: String,
    extension_type: ExtensionTypes,
    width: u32,
    height: u32,
    data: String
}

impl FileObject {
    pub fn new_empty() -> Self {
        FileObject {
            loaded: false,
            file_address: String::from(""),
            extension_type: ExtensionTypes::NA,
            width: 0,
            height: 0,
            data: String::from("")
        }
    }

    pub fn new_from_base64(addr: String, data_b64: String) -> Option<Self> {
        Some(FileObject {
            loaded: true,
            file_address: addr,
            extension_type: ExtensionTypes::NA,
            width: 0,
            height: 0,
            data: String::from("")
        })
    }
}
