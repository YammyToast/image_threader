use crate::ui::props::{ExtensionTypes, FileObject};
use image::{GenericImage, GenericImageView, DynamicImage, ImageResult, Rgb, RgbImage, ImageDecoder};
use image::codecs::jpeg::JpegDecoder;
use image::codecs::png::PngDecoder;
use std::collections::{HashMap, VecDeque};
use zerocopy::{AsBytes, FromBytes};



use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use std::rc::Rc;
use std::time::{Duration, SystemTime};

type ThreadFunction =
    fn(&dyn GenericImageView<Pixel = Rgb<u8>>, &ThreadObject) -> VecDeque<Rc<ThreadPoint>>;
type CompositeThreadVector = Vec<(Rgb<u8>, VecDeque<Rc<ThreadPoint>>)>;
const DEFAULT: Vec<(&str, &ThreadFunction)> = Vec::new();

pub fn main() {
    let file_obj = get_test_file_obj();
    let thread_obj = ThreadObject::new_from_fileobject(&file_obj);

    let file_obj_2 = get_test_file_obj_two();
    let thread_obj_2 = ThreadObject::new_from_fileobject(&file_obj_2);
    // println!("{:?}", thread_obj.derived_fileobject.data_url)
}

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
        data_url: String::from(include_str!("testobj.txt")),
    }
}

fn get_test_file_obj_two() -> FileObject {
    FileObject {
        loaded: true,
        file_address: String::from(""),
        extension_type: ExtensionTypes::PNG,
        width: 728,
        height: 410,
        output_width: 728,
        output_height: 410,
        flip_x: false,
        flip_y: false,
        data_url: String::from(include_str!("testobj2.txt"))
    }

}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ThreadObjectError {
    InvalidDataURL,
    InvalidImageData,
    FormatConversionFail,
    VerifyFailed,
    UnknownAlgorithmName,
    AlgorithmTimedOut,
    UnsupportedFileType,
}

struct ThreadObject<'a> {
    derived_fileobject: &'a FileObject,
    data: HashMap<&'a str, ThreadList<'a>>,
    generated_count: u32,
    color_depth: u8,
    image: RgbImage,
}

impl<'a> ThreadObject<'a> {
    fn decode_png(bytes: Vec<u8>, output_width: &u32, output_height: &u32) -> Result<DynamicImage, ThreadObjectError> {
        let mut slice_bytes = bytes.as_slice();
        let mut decoder = match PngDecoder::new(slice_bytes) {
            Ok(dec) => dec,
            Err(_) => return Err(ThreadObjectError::InvalidImageData)
        };
        // decoder.scale(*output_width as u16, *output_height as u16);
        let img: DynamicImage = DynamicImage::from_decoder(decoder).unwrap();
        img.save("test.jpg").unwrap();
        Ok(img)
    }

    fn decode_jpg(bytes: Vec<u8>, output_width: &u32, output_height: &u32) -> Result<DynamicImage, ThreadObjectError>
    {
        let mut slice_bytes = bytes.as_slice();
        let mut decoder = match JpegDecoder::new(slice_bytes) {
            Ok(dec) => dec,
            Err(_) => return Err(ThreadObjectError::InvalidImageData)

        };
        decoder.scale(*output_width as u16, *output_height as u16);
        // let mut buf: Vec<u16> = vec![0; (decoder.total_bytes()/2).try_into().unwrap()];
        let img: DynamicImage = DynamicImage::from_decoder(decoder).unwrap();
        img.save("test.png").unwrap();
        Ok(img)        
    }

    pub fn decode_data_url(
        data_url: &String,
        file_extension: &ExtensionTypes,
        width: &u32,
        height: &u32,
        output_width: &u32,
        output_height: &u32,
    ) -> Result<RgbImage, ThreadObjectError> {
        let parts = data_url.split(",");
        let collection = parts.collect::<Vec<&str>>();
        let base64 = match collection.get(1) {
            Some(val) => val,
            None => return Err(ThreadObjectError::InvalidDataURL),
        };

        let bytes: Vec<u8> = general_purpose::STANDARD.decode(base64).unwrap();

        let decode_result = match &file_extension {
            ExtensionTypes::PNG => ThreadObject::decode_png(bytes, output_width, output_height),
            ExtensionTypes::JPG => ThreadObject::decode_jpg(bytes, output_width, output_height),
            _ => return Err(ThreadObjectError::UnsupportedFileType),
        };

        let decode_img: DynamicImage = match decode_result {
            Ok(img) => img,
            Err(e) => return Err(e)            
        };

        let conversion_image: RgbImage = decode_img.into_rgb8();
        conversion_image.save("burger.png").unwrap();
        Ok(conversion_image)
    }

    pub fn new_from_fileobject(file_obj: &'a FileObject) -> Result<Self, ThreadObjectError> {
        let buffer = RgbImage::new(file_obj.output_width, file_obj.output_height);
        ThreadObject::decode_data_url(
            &file_obj.data_url,
            &file_obj.extension_type,
            &file_obj.width,
            &file_obj.height,
            &file_obj.output_width,
            &file_obj.output_height,
        );

        Ok(ThreadObject {
            derived_fileobject: file_obj,
            data: HashMap::new(),
            generated_count: 0,
            color_depth: 255,
            image: buffer,
        })
    }

    pub fn is_loaded(&self) -> bool {
        match self.data.keys().count() {
            0 => false,
            _ => true,
        }
    }

    pub fn execute_algorithm(
        &self,
        _key: &str,
    ) -> Result<CompositeThreadVector, ThreadObjectError> {
        if self.data.contains_key(_key) == false {
            return Err(ThreadObjectError::UnknownAlgorithmName);
        }
        Err(ThreadObjectError::InvalidImageData)
    }
}

struct ThreadList<'a> {
    image: &'a RgbImage,
    linked_thread_object: &'a ThreadObject<'a>,
    points: CompositeThreadVector,
    pub algorithm: ThreadFunction,
    pub algorithm_last_duration: Duration,
}

impl<'a> ThreadList<'a> {
    pub fn run_algorithm(&mut self) {
        let start_time = SystemTime::now();
        let view: &dyn GenericImageView<Pixel = Rgb<u8>> = self.image;
        (self.algorithm)(view, self.linked_thread_object);
        self.algorithm_last_duration = start_time.elapsed().unwrap();
    }
}

struct ThreadPoint {
    pub x: u32,
    pub y: u32,
    pub color_val: Rgb<u8>,
    next_point: Option<Rc<ThreadPoint>>,
    pub dist_next_point: u32,
}
