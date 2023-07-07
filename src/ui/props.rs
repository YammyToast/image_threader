#[derive(Debug)]
pub enum WindowState {
    MainMenu,
    ConvertUpload,
    ViewUpload,
    ConfigureConvert,
    Loading,
    ConvertRender

}

pub struct MenuProps {
    state: WindowState,
    stateAccess: fn(WindowState)

}