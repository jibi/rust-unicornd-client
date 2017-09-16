//           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                   Version 2, December 2004
//
// Copyright (C) 2017 Gilberto Bertin <me@jibi.io>
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.
//

use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::io;

const BRIGHTNESS: u8 = 0;
const SET_PIXEL: u8 = 1;
const SET_ALL_PIXELS: u8 = 2;
const SHOW: u8 = 3;

#[repr(C,packed)]
#[derive(Copy,Clone)]
pub struct Pos {
    pub x: u8,
    pub y: u8
}

#[repr(C,packed)]
#[derive(Copy,Clone)]
pub struct Pixel {
    pub g: u8,
    pub r: u8,
    pub b: u8
}

#[repr(C,packed)]
struct SetBrightnessCmd{
    code: u8,
    brightness: u8
}

#[repr(C,packed)]
struct SetPixelCmd {
    code: u8,
    pos: Pos,
    col: Pixel
}

#[repr(C,packed)]
struct SetAllPixelsCmd {
    code: u8,
    pixels: [Pixel; 64]
}

pub struct UnicorndClient {
    sd: UnixStream
}

impl UnicorndClient {
    pub fn new(path: String) -> Option<Self> {
        let p = path.clone();
        let sd = match UnixStream::connect(path) {
            Ok(sd) => sd,
            Err(e) => {
                eprintln!("Cannot connect to {}: {}", p, e);
                return None
            },
        };

        return Some(UnicorndClient{sd});
    }

    pub fn set_brightness(&mut self, brightness: u8) -> io::Result<()> {
        self.write(&SetBrightnessCmd{
            code: BRIGHTNESS,
            brightness: brightness
        })
    }

    pub fn set_pixel(&mut self, pos: Pos, col: Pixel) -> io::Result<()> {
        self.write(&SetPixelCmd{
            code: SET_PIXEL,
            pos: pos,
            col: col
        })
    }

    pub fn set_all_pixels(&mut self, pixels: [Pixel; 64]) -> io::Result<()> {
        self.write(&SetAllPixelsCmd{
            code: SET_ALL_PIXELS,
            pixels: pixels
        })
    }

    pub fn show(&mut self) -> io::Result<()> {
        self.write(&SHOW)
    }

    fn write<T: Sized>(&mut self, p: &T) -> io::Result<()> {
        let buf = unsafe {
            ::std::slice::from_raw_parts(
                (p as *const T) as *const u8,
                ::std::mem::size_of::<T>()
                )
        };

        self.sd.write_all(buf)
    }
}
