use crate::ui::props::{FileObject, ExtensionTypes};
use std::collections::{HashMap, VecDeque};
use image::{Rgb, RgbImage, GenericImage, GenericImageView};
use std::rc::Rc;
use std::time::{Duration, SystemTime};


pub fn main() {
    let file_obj = get_test_file_obj();
    let thread_obj = ThreadObject::new_from_fileobject(&file_obj);
    println!("{:?}", thread_obj.derived_fileobject.extension_type)
}  

type ThreadFunction = fn(&dyn GenericImageView<Pixel=Rgb<u8>>, &ThreadObject) -> VecDeque<Rc<ThreadPoint>>;

fn get_test_file_obj() -> FileObject {
    FileObject {
        loaded: true,
        file_address: String::from(""),
        extension_type: ExtensionTypes::JPG,
        width: 728,
        height: 410,
        output_width: 728,
        output_height: 410,
        flip_x: false,
        flip_y: false,
        data_url: String::from(include_str!("testobj.txt"))   
    }
}

struct ThreadObject<'a> {
    derived_fileobject: &'a FileObject,
    data: HashMap<&'a str, ThreadList<'a>>,
    generated_count: u32,
    color_depth: u8,
    image: RgbImage
}

impl<'a> ThreadObject<'a> {
    pub fn new_from_fileobject(file_obj: &'a FileObject) -> Self {
        let buffer = RgbImage::new(10, 10);
        ThreadObject {
            derived_fileobject: file_obj,
            data: HashMap::new(),
            generated_count: 0,
            color_depth: 255,
            image: buffer
        }
    }

    pub fn is_loaded(&self) -> bool {
        match self.data.keys().count() {
            0 => false,
            _ => true
        }
    } 
}

struct ThreadList<'a> {
    image: &'a RgbImage,
    linked_thread_object: &'a ThreadObject<'a>,
    points: VecDeque<Rc<ThreadPoint>>,
    pub algorithm: ThreadFunction,
    pub algorithm_last_duration: Duration,
}

impl<'a> ThreadList<'a> {
    pub fn run_algorithm(&mut self) {
        let start_time = SystemTime::now();
        let view: &dyn GenericImageView<Pixel=Rgb<u8>> = self.image;
        (self.algorithm)(view, self.linked_thread_object);
        self.algorithm_last_duration = start_time.elapsed().unwrap();
    }

}

struct ThreadPoint {
    pub x: u32,
    pub y: u32,
    pub color_val: Rgb<u8>, 
    next_point: Option<Rc<ThreadPoint>>,
    pub dist_next_point: u32
}