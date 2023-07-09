use dioxus::prelude::*;

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
    loaded: bool,
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

    pub fn new_file(addr: String, data: String) -> Option<Self> {
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
