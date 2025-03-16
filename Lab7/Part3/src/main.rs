use rayon::prelude::*;
#[macro_use]
extern crate error_chain; // The crate for this is written as error-chain.
extern crate image;
extern crate num;
extern crate num_cpus;
extern crate threadpool;

// These are the dependencies and their versions:
// error-chain = "0.12.4"
// image = "0.25.4"
// num = "0.4.3"
// num_cpus = "1.16.0"
// threadpool = "1.8.1"

use image::{ImageBuffer, Pixel, Rgb};
use num::complex::Complex;
use std::sync::mpsc::{channel, RecvError};

error_chain! {
    foreign_links {
        MpscRecv(RecvError);
        Io(std::io::Error);
    }
}

fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
    let wave = wavelength as f32;
    let (r, g, b) = match wavelength {
        380..=439 => ((440. - wave) / (440. - 380.), 0.0, 1.0),
        440..=489 => (0.0, (wave - 440.) / (490. - 440.), 1.0),
        490..=509 => (0.0, 1.0, (510. - wave) / (510. - 490.)),
        510..=579 => ((wave - 510.) / (580. - 510.), 1.0, 0.0),
        580..=644 => (1.0, (645. - wave) / (645. - 580.), 0.0),
        645..=780 => (1.0, 0.0, 0.0),
        _ => (0.0, 0.0, 0.0),
    };
    
    let factor = match wavelength {
        380..=419 => 0.3 + 0.7 * (wave - 380.) / (420. - 380.),
        701..=780 => 0.3 + 0.7 * (780. - wave) / (780. - 700.),
        _ => 1.0,
    };
    
    let (r, g, b) = (
        normalize(r, factor),
        normalize(g, factor),
        normalize(b, factor),
    );
    
    Rgb::from_channels(r, g, b, 0)
}

fn julia(x: u32, y: u32, width: u32, height: u32, max_iter: u32) -> u32 {
    let width = width as f32;
    let height = height as f32;
    let c = Complex::new(-0.8, 0.156);
    let mut z = Complex {
        // Scale and translate the point to image coordinates
        re: 3.0 * (x as f32 - 0.5 * width) / width,
        im: 2.0 * (y as f32 - 0.5 * height) / height,
    };
    
    let mut i = 0;
    for t in 0..max_iter {
        if z.norm() >= 2.0 {
            break;
        }
        z = z * z + c;
        i = t;
    }
    i
}

fn mandelbrot(x: u32, y: u32, width: u32, height: u32, max_iter: u32) -> u32 {
    let width = width as f32;
    let height = height as f32;
    
    let c = Complex {
        // Scale and translate the point to image coordinates
        re: 3.5 * (x as f32 - 0.5 * width) / width,
        im: 2.0 * (y as f32 - 0.5 * height) / height,
    };

    let mut z = Complex::new(0.0, 0.0);
    
    let mut i = 0;
    for t in 0..max_iter {
        if z.norm() >= 2.0 {
            break;
        }
        z = z * z + c;
        i = t;
    }
    i
}

fn normalize(color: f32, factor: f32) -> u8 {
    ((color * factor).powf(0.8) * 255.) as u8
}

fn main() -> Result<()> {
    let (width, height) = (1920, 1080);
    let mut img = ImageBuffer::new(width, height);
    let iterations = 300;
    let (tx, rx) = channel();
    
    for y in 0..height {
        let tx = tx.clone();
        //each x in width done in parallel
        //pool.execute(move || {
        //    for x in 0..width {
        //        let i = mandelbrot(x, y, width, height, iterations);
        //        let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
        //        tx.send((x, y, pixel)).expect("Could not send data!");
        //    }
        //});
        (0..width).into_par_iter().for_each(|x| {
            let i = mandelbrot(x, y, width, height, iterations);
            let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
            tx.send((x, y, pixel)).expect("Could not send data!");
        });
    }
    
    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv()?;
        img.put_pixel(x, y, pixel);
    }
    
    let _ = img.save("output.png");
    Ok(())
}
