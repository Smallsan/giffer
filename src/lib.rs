use image::{AnimationDecoder, ImageBuffer, Rgba, RgbaImage};
use image::codecs::gif::{GifDecoder, GifEncoder};
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufWriter};


pub fn load_gif<P: AsRef<Path>>(path: P) -> Result<Vec<RgbaImage>, image::ImageError> {
    let file = File::open(path)?;
    let decoder = GifDecoder::new(BufReader::new(file))?;
    let frames = decoder.into_frames().collect_frames()?;
    Ok(frames.into_iter().map(|frame| frame.into_buffer()).collect())
}