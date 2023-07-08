use dioxus::prelude::*;

#[derive(Debug)]
pub enum WindowTypes {
    MainMenu,
    ConvertUpload,
    ViewUpload,
    ConfigureConvert,
    Loading,
    ConvertRender

}


pub struct FileObject {
    file_address: str

}