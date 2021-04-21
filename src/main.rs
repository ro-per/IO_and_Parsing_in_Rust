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
fn get_char(cursor: &mut Cursor<Vec<u8>>) -> Result<Image, std::io::Error> {
    let mut image = Image { 
        width: 0,
        height: 0,
        pixels: vec![]
    };

    /* INLEZEN VAN HET TYPE */
    let mut header: [u8;2]=[0;2]; // inlezen van karakters
    cursor.read(&mut header)?; // ? geeft error terug mee met result van de functie
    match &header{ // & dient voor slice van te maken
        b"P6" => println!("P6 image"),  // b zorgt ervoor dat je byte string hebt (u8 slice)
        _ => panic!("Not an P6 image")  //_ staat voor default branch
    }

    /* INLEZEN VAN BREEDTE EN HOOGTE */
    image.width=read_number(cursor)?;
    image.height=read_number(cursor)?;
    let colourRange = read_number(cursor)?;

    /* eventuele whitespaces na eerste lijn */
    consume_whitespaces(cursor)?;

    /* body inlezen */

    for _ in 0.. image.height{
        let mut row = Vec::new();
        for _ in 0..image.width{
            let red = cursor.read_u8()?;
            let green = cursor.read_u8()?;
            let blue = cursor.read_u8()?;

            //if red < 255 && green < 255{
                row.push(Pixel{r:red,g:green,b:blue});
            //}
        }
        image.pixels.push(row);
    }




    // TODO: Parse the image here

    Ok(image)
}




fn decode_ppm_image(cursor: &mut Cursor<Vec<u8>>) -> Result<Image, std::io::Error> {
    let mut image = Image { 
        width: 0,
        height: 0,
        pixels: vec![]
    };

    /* INLEZEN VAN HET TYPE */
    let mut header: [u8;2]=[0;2]; // inlezen van karakters
    cursor.read(&mut header)?; // ? geeft error terug mee met result van de functie
    match &header{ // & dient voor slice van te maken
        b"P6" => println!("P6 image"),  // b zorgt ervoor dat je byte string hebt (u8 slice)
        _ => panic!("Not an P6 image")  //_ staat voor default branch
    }

    /* INLEZEN VAN BREEDTE EN HOOGTE */
    image.width=read_number(cursor)?;
    image.height=read_number(cursor)?;
    //let colourRange = read_number(cursor)?;

    /* eventuele whitespaces na eerste lijn */
    consume_whitespaces(cursor)?;

    /* body inlezen */

    for _ in 0.. image.height{
        let mut row = Vec::new();
        for _ in 0..image.width{
            let red = cursor.read_u8()?;
            let green = cursor.read_u8()?;
            let blue = cursor.read_u8()?;

            row.push(Pixel{r:red,g:green,b:blue});
            

        }
        image.pixels.push(row);
    }




    // TODO: Parse the image here

    Ok(image)
}

fn read_number(cursor: &mut Cursor<Vec<u8>>)-> Result<u32,std::io::Error>{
    consume_whitespaces(cursor)?;

    let mut buff: [u8;1] = [0];
    let mut v = Vec::new(); // vector waar je bytes gaat in steken

    loop{
        cursor.read(& mut buff)?;
        match buff[0]{
            b'0'..= b'9' => v.push(buff[0]),
            b' ' | b'\n' | b'\r' | b'\t' => break,
            _ => panic!("Not a valid image")
        }
    }
    // byte vector omzetten
    let num_str: &str = std::str::from_utf8(&v).unwrap(); // unwrap gaat ok value er uit halen als het ok is, panic als het niet ok is
    let num =num_str.parse::<u32>().unwrap(); // unwrap dient voor errors

    // return
    Ok(num)

    //return Ok(num); andere mogelijke return
}

fn consume_whitespaces (cursor: &mut Cursor<Vec<u8>>)-> Result<(),std::io::Error>{ //Result<() : de lege haakjes betekend  niks returnen
    let mut buff: [u8;1] = [0];

    loop{
        cursor.read(& mut buff)?;
        match buff[0]{
            b' ' | b'\n' | b'\r' | b'\t' => println!("Whitespace"),
            _ => { // je zit eigenlijk al te ver nu !!! zet cursor 1 terug
                cursor.seek(SeekFrom::Current(-1))?;
                break;
            }
        }
    }
    Ok(()) // () : de lege haakjes betekend  niks returnen

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
    /* let image = match decode_ppm_image(&mut cursor) {
        Ok(img) => img,
        Err(why) => panic!("Could not parse PPM file - Desc: {}", why),
    }; */

    let image = match get_char(&mut cursor) {
        Ok(img) => img,
        Err(why) => panic!("Could not parse PPM file - Desc: {}", why),
    };

    show_image(&image);
}