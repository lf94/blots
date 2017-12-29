extern crate image;

#[macro_use]
extern crate lazy_static;

use std::process::{
  exit
};
use std::env;
use std::io::{
  Read
};
use std::fs::File;
use std::path::Path;
use image::{
  RgbImage,
  Rgb
};

const A4_WIDTH_INCHES:f32 = 8.3;
const A4_RATIO:f32 = 1.4142;
const DPI:u32 = 300;
const A4_PIXEL_WIDTH:usize = (A4_WIDTH_INCHES * DPI as f32) as usize;
const A4_PIXEL_HEIGHT:usize = (A4_PIXEL_WIDTH as f32 * A4_RATIO) as usize;

enum Orientation {
  UP,
  DOWN
}

lazy_static! {
  static ref BITS_TO_COLOR:[Rgb<u8>; 4] = [
      Rgb([255u8,   0u8,   0u8]),
      Rgb([  0u8, 255u8,   0u8]),
      Rgb([  0u8,   0u8, 255u8]),
      Rgb([255u8, 255u8, 255u8])
  ];
}

struct Point {
  x: u32,
  y: u32
}

trait BlotsEncoder {
  fn draw_alignment_component(position: Point, blots_image: &mut RgbImage) -> Point;
  fn draw_data_blocks(data: &[u8], blots_image: &mut RgbImage) -> ();
  fn draw_data_block(orientation: Orientation, data: [u8; 3], pos: Point, ib: &mut RgbImage) -> Point;
  fn new(data: &[u8]) -> RgbImage;
}

struct Blots {
}

impl Blots {
  fn new(data: &[u8]) -> RgbImage {
    let white_image = vec![255u8; A4_PIXEL_WIDTH * A4_PIXEL_HEIGHT * 4];
    let mut blots_image = RgbImage::from_raw(A4_PIXEL_WIDTH as u32, A4_PIXEL_HEIGHT as u32, white_image).unwrap();
    let mut position = Point {
      x: 0,
      y: 0
    };

    position = Blots::draw_alignment_component(position, &mut blots_image);
    position = Point {
      x: 0,
      y: position.y
    };

    Blots::draw_data_blocks(data, position, &mut blots_image);
    blots_image
  }

  fn draw_alignment_component(position: Point, ib: &mut RgbImage ) -> Point {
    let black = Rgb([0u8, 0u8, 0u8]);

    for i in 0..3 {
      Blots::straight_vertical_line(
        ib,
        black,
        Point { x: position.x + i, y: position.y },
        Point { x: position.x + i, y: position.y + 16 }
      );
    }

    for i in 0..3 {
      Blots::straight_vertical_line(
        ib,
        black,
        Point { x: position.x + 4 + 4 + i, y: position.y },
        Point { x: position.x + 4 + 4 + i, y: position.y + 16 }
      );
    }

    for i in 0..3 {
      Blots::draw_line(
        ib,
        black,
        Point { x: position.x + 4 + 4, y: position.y + 16 + i },
        Point { x: position.x + 4 + 4 + 16, y: position.y + 16 + i }
      );
    }

    Point { x: position.x, y: position.y + 16 + 3 }
  }

  fn straight_vertical_line(ib: &mut RgbImage, color: Rgb<u8>, p1: Point, p2: Point) {
    let mut current_y = p1.y;
    while current_y <= p2.y {
      ib.put_pixel(p1.x, current_y, color);
      current_y = current_y + 1;
    }
  }

  fn draw_line(ib: &mut RgbImage, color: Rgb<u8>, p1: Point, p2: Point) {
    let deltas = Point {
      x: p2.x - p1.x,
      y: p2.y - p1.y
    };

    let mut current_x = p1.x;
    while current_x <= p2.x {
      let current_y = p1.y + deltas.y * (current_x - p1.x) / deltas.x;

      ib.put_pixel(current_x, current_y, color);
      current_x = current_x + 1;
    }
  }

  fn draw_data_blocks(data: &[u8], position: Point, ib: &mut RgbImage) -> () {
    let mut next_position = position;

    for byte in data.iter() {
      let bits_7_6 = (byte & 0b11000000) >> 6;
      let bits_5_4 = (byte & 0b00110000) >> 4;
      let bits_3_2 = (byte & 0b00001100) >> 2;
      let bits_1_0 = (byte & 0b00000011) >> 0;
      
      let parity_bits_7_4 = bits_7_6 ^ bits_5_4;
      let parity_bits_3_0 = bits_3_2 ^ bits_1_0;

      let block_1 = [bits_7_6, bits_5_4, parity_bits_7_4];
      let block_2 = [bits_3_2, bits_1_0, parity_bits_3_0];

      next_position = Blots::draw_data_block(Orientation::UP,   block_1, next_position, ib);
      next_position = Blots::draw_data_block(Orientation::DOWN, block_2, next_position, ib);

      if next_position.x + 3 >= ib.width() {
        next_position.y += 1;
        next_position.x = 0;
      }
    }
  }

  fn draw_data_block(orientation: Orientation, data: [u8; 3], pos: Point, ib: &mut RgbImage) -> Point {
    match orientation {
      Orientation::UP => {
        ib.put_pixel(pos.x,     pos.y,     BITS_TO_COLOR[data[0] as usize]);
        ib.put_pixel(pos.x + 1, pos.y,     BITS_TO_COLOR[data[1] as usize]);
        ib.put_pixel(pos.x,     pos.y + 1, BITS_TO_COLOR[data[2] as usize]);

        Point { x: pos.x + 1, y: pos.y + 1 }
      },
      Orientation::DOWN => {
        ib.put_pixel(pos.x,     pos.y,     BITS_TO_COLOR[data[0] as usize]);
        ib.put_pixel(pos.x,     pos.y,     BITS_TO_COLOR[data[0] as usize]);
        ib.put_pixel(pos.x + 1, pos.y,     BITS_TO_COLOR[data[1] as usize]);
        ib.put_pixel(pos.x,     pos.y - 1, BITS_TO_COLOR[data[2] as usize]);

        Point { x: pos.x + 1, y: pos.y - 1 }
      }
    }
  }
}

fn main() {
  let switch         = env::args().nth(1).expect("Specify --encode or --decode.");
  let filename       = env::args().nth(2).expect("Please provide a filename.");

  let mut data:Vec<u8> = Vec::new();
  let mut file         = File::open(filename.clone()).expect("Could not open provided file.");
  file.read_to_end(&mut data).expect("Error occurred reading file.");

  if switch == "--encode" {
    let paper_pixel_size = A4_PIXEL_WIDTH * A4_PIXEL_HEIGHT;
    let file_pixel_size  = data.len() * 3;
    if file_pixel_size > paper_pixel_size {
      println!("File exceeds A4 paper size. {} px vs {} px respectively.",
        file_pixel_size,
        paper_pixel_size
      );
      exit(1);
    }

    let blots_filename = filename.clone() + ".blots.png";

    Blots
    ::new(&data)
    .save(Path::new(blots_filename.as_str()))
    .expect("Could not save result to file.");
  }

  if switch == "--decode" {
    println!("Decoding not supported yet.");
    exit(1);
  }

  exit(0);

