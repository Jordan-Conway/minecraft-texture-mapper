use std::collections;
use image::{Rgb};
use std::env;

fn main() {
    return;
}

pub struct PixelMapping {
    mapping: collections::HashMap<image::Rgb<u8>, Rgb<u8>>
}

pub trait TransformPixel{
    fn transform_pixel(&self, pixel: image::Rgb<u8>) -> Option<&image::Rgb<u8>>;
}

impl TransformPixel for PixelMapping {
    fn transform_pixel(&self, pixel: image::Rgb<u8>) -> Option<&image::Rgb<u8>>{  
        return self.mapping.get(&pixel);
    }
}