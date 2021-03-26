use std::io::{Error, ErrorKind};
use std::path::Path;
use std::fs::File;
use std::io::{Read, Cursor};
use byteorder::ReadBytesExt;
use std::io::{Seek, SeekFrom};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use shuteye::sleep;
use std::time::Duration;

#[derive(Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Vec<Pixel>>
}

fn show_image(image: &Image) {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let display_mode = video_subsystem.current_display_mode(0).unwrap();

    let w = match display_mode.w as u32 > image.width {
        true => image.width,
        false => display_mode.w as u32
    };
    let h = match display_mode.h as u32 > image.height {
        true => image.height,
        false => display_mode.h as u32
    };
    
    let window = video_subsystem
        .window("Image", w, h)
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();
    let black = sdl2::pixels::Color::RGB(0, 0, 0);

    let mut event_pump = sdl.event_pump().unwrap();

    // render image
    canvas.set_draw_color(black);
    canvas.clear();

    for r in 0..image.height {
        for c in 0..image.width {
            let pixel = &image.pixels[r as usize][c as usize];
            canvas.set_draw_color(Color::RGB(pixel.r as u8, pixel.g as u8, pixel.b as u8));
            canvas.fill_rect(Rect::new(c as i32, r as i32, 1, 1)).unwrap();
        }
    }

    canvas.present();

    'main: loop {        
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }
        sleep(Duration::new(0, 250000000));
    }
}

fn decode_ppm_image(cursor: &mut Cursor<Vec<u8>>) -> Result<Image, std::io::Error> {
    let mut image = Image { 
        width: 0,
        height: 0,
        pixels: vec![]
    };

    // TODO: Parse the image here

    Ok(image)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Syntax: {} <filename>", args[0]);
        return;
    }

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open file: {} (Reason: {})", 
            display, why),
        Ok(file) => file
    };

    // read the full file into memory. panic on failure
    let mut raw_file = Vec::new();
    file.read_to_end(&mut raw_file).unwrap();

    // construct a cursor so we can seek in the raw buffer
    let mut cursor = Cursor::new(raw_file);
    let image = match decode_ppm_image(&mut cursor) {
        Ok(img) => img,
        Err(why) => panic!("Could not parse PPM file - Desc: {}", why),
    };

    show_image(&image);
}