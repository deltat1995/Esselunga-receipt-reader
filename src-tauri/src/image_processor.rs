use std::{collections::HashMap, io::Cursor};

use base64::prelude::*;
use image::{self, DynamicImage, ImageFormat};
use rusty_tesseract::{image_to_string, Args, Image};

pub enum CharList {
    WORD,
    PRICE,
}

impl CharList {
    pub fn value(&self) -> &str {
        match *self {
            CharList::WORD => {
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890.+- %)/"
            }
            CharList::PRICE => "0123456789,-S",
        }
    }
}

pub fn extract_text_from_image(dyn_img: &DynamicImage, char_list: &str, dpi: i32) -> String {
    let img = Image::from_dynamic_image(dyn_img).unwrap();
    let my_args = Args {
        //model language (tesseract default = 'eng')
        //available languages can be found by running 'rusty_tesseract::get_tesseract_langs()'
        lang: "ita".to_owned(),

        //map of config variables
        //this example shows a whitelist for the normal alphabet. Multiple arguments are allowed.
        //available arguments can be found by running 'rusty_tesseract::get_tesseract_config_parameters()'
        config_variables: HashMap::from([("tessedit_char_whitelist".into(), char_list.into())]),
        dpi: Some(dpi), // specify DPI for input image
        psm: Some(4), // define page segmentation mode 6 (i.e. "Assume a single uniform block of text")
        oem: Some(1), // define optical character recognition mode 3 (i.e. "Default, based on what is available")
    };

    return image_to_string(&img, &my_args).unwrap();
}

pub fn generate_img_src_from(image: &DynamicImage, format: ImageFormat) -> Result<String, String> {
    let mut buf: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut buf), format)
        .map_err(|_| "unexpected error to read the image data".to_string())?;
    return Ok(format!(
        "data:{};base64,{}",
        format.to_mime_type(),
        BASE64_STANDARD.encode(&buf)
    ));
}
