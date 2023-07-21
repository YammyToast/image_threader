use dioxus::prelude::*;
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};


#[derive(Debug)]
pub enum WindowTypes {
    MainMenu,
    ConvertUpload,
    ViewUpload,
    ConfigureConvert,
    ConvertRender,
}

#[derive(Debug, Clone)]
pub enum ExtensionTypes {
    PNG,
    JPG,
    NA,
}

#[derive(Debug, Clone)]
pub struct FileObject {
    pub loaded: bool,
    pub file_address: String,
    extension_type: ExtensionTypes,
    pub width: u32,
    pub height: u32,
    pub output_width: u32,
    pub output_height: u32,
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
            output_width: 0,
            output_height: 0,
            data_url: String::from("")
        }
    }

    pub fn new_from_url(addr: String, data_url: String, height: u32, width: u32) -> Option<Self> {
        let parts = data_url.split(",");
        let collection = parts.collect::<Vec<&str>>();
        // println!("{:?}", collection);
        let bytes = general_purpose::STANDARD.decode(collection[1]).unwrap();
        println!("{:?}", bytes.len());
        Some(FileObject {
            loaded: true,
            file_address: addr,
            extension_type: ExtensionTypes::NA,
            width: width,
            height: height,
            output_width: width,
            output_height: height,
            data_url: data_url
        })
    }


}
