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

extern crate unicornd_client;

use std::thread::sleep;
use std::time::Duration;
use unicornd_client::{UnicorndClient, Pos, Pixel};

fn gen_solid(pixel: Pixel) -> [Pixel; 64] {
    let mut pixels = [Pixel{r:0, g:0, b:0}; 64];
    for x in 0..64 {
        pixels[x] = pixel.clone()
    }
    pixels
}

fn main() {
    let mut u = UnicorndClient::new("/var/run/unicornd.socket".to_string()).unwrap();
    u.set_brightness(128).unwrap();

    let rgb = [
        Pixel{r: 255, g:   0, b:   0},
        Pixel{r:   0, g: 255, b:   0},
        Pixel{r:   0, g:   0, b: 255}
    ];

    loop {
        for p in rgb.iter() {
            u.set_all_pixels(gen_solid(*p)).unwrap();
            for x in 3..5 {
                for y in 3..5 {
                    u.set_pixel(Pos{x, y}, Pixel{r: 255, g: 255, b: 255}).unwrap();
                }
            }

            u.show().unwrap();
            sleep(Duration::new(1, 0));
        }
    }
}
