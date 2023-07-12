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
    pub data_url: String,
}

impl FileObject {
    pub fn new_empty() -> Self {
        FileObject {
            loaded: false,
            file_address: String::from(""),
            extension_type: ExtensionTypes::NA,
            width: 0,
            height: 0,
            data_url: String::from("")
        }
    }

    pub fn new_from_url(addr: String, data_url: String) -> Option<Self> {
        let parts = data_url.split(",");
        let collection = parts.collect::<Vec<&str>>();
        // println!("{:?}", collection);
        let bytes = general_purpose::STANDARD.decode(collection[1]).unwrap();
        println!("{:?}", bytes.len());
        Some(FileObject {
            loaded: true,
            file_address: addr,
            extension_type: ExtensionTypes::NA,
            width: 0,
            height: 0,
            data_url: data_url
        })
    }

}
