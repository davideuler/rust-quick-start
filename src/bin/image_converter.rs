//! An example of convert image from an format(webp, jpg, bmp, git, etc. ) to another format (png).
extern crate image;

use std::{env,fs};
//  in-memory object that can be used as a Reader, Writer, or Seek in Rust? 
// https://stackoverflow.com/questions/41069865/how-to-create-an-in-memory-object-that-can-be-used-as-a-reader-writer-or-seek
use std::io::{Cursor, Write}; 
use std::fs::File;
use std::path::Path;

use image::{GenericImageView, ImageFormat};

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    // option 1: open DynamicImage from path
    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    // let im = image::open(&Path::new(&file)).unwrap();

    // option 2: read file from path into bytes:
    let data = fs::read(file.clone()).unwrap();
    
    // let buffer: &[u8] = "".as_bytes();
    let im =  image::load_from_memory(data.as_slice()).unwrap();

    // The dimensions method returns the images width and height
    println!("dimensions {:?}", im.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", im.color());

    let fout = &mut File::create(&Path::new(&format!("{}.png", file))).unwrap();

    // option 1: write to file directly
    // Write the contents of this image to the Writer in PNG format.
    // im.write_to(fout, ImageFormat::Png).unwrap();

    // option 2: write to memory buffer firstly
    // Write the contents of this image to memory buffer in PNG format.
    let mut buffer = Cursor::new(Vec::new());

    // let mut buffer: Vec<u8> = Vec::new();
    im.write_to(&mut buffer, ImageFormat::Png).unwrap();

    // writer buffer bytes to file
    fout.write_all(buffer.get_ref()).unwrap();
    
}
