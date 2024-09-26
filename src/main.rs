use std::collections::HashMap;

use clap::Parser;
use image::{DynamicImage, ImageReader};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'i', long = "input")]
    input: std::path::PathBuf,
    #[arg(short = 'o', long = "output")]
    output: std::path::PathBuf,
}

fn resize_for_console(img: DynamicImage) -> DynamicImage {
    img.resize(80, 24, image::imageops::FilterType::Lanczos3)
}

fn setup_value_conversion() -> HashMap<u8, char> {
    let mut ascii_map = HashMap::new();
    ascii_map.insert(0, ' ');
    ascii_map.insert(1, '.');
    ascii_map.insert(2, '-');
    ascii_map.insert(3, '=');
    ascii_map.insert(4, '+');
    ascii_map.insert(5, '*');
    ascii_map.insert(6, '#');
    ascii_map.insert(7, '@');

    ascii_map
}

fn convert_pixel(value: u8, conversion_map: &HashMap<u8, char>) -> char {
    let max_supported_value: f32 = conversion_map.keys().max().unwrap().clone().into();

    let search_value = ((value as f32 / 255.0) * max_supported_value).round() as u8;

    conversion_map.get(&search_value).unwrap().clone()
}

fn main() {
    let args = Cli::parse();

    let conversion_map = setup_value_conversion();

    let img = ImageReader::open(&args.input).unwrap().decode().unwrap();

    let resized_image = resize_for_console(img);

    let values = resized_image.into_luma8().into_vec();

    let converted_values = values
        .into_iter()
        .map(|v| convert_pixel(v, &conversion_map));

    println!("image luma values: {:?})", converted_values);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_zero() {
        let conversion_map = setup_value_conversion();
        assert!(convert_pixel(0, &conversion_map) == ' ');
    }

    #[test]
    fn convert_max() {
        let conversion_map = setup_value_conversion();
        assert!(convert_pixel(255, &conversion_map) == '@');
    }

    #[test]
    fn convert_vec() {
        let conversion_map = setup_value_conversion();
        let values = vec![0, 5, 40, 128, 255];
        assert_eq!(
            values
                .into_iter()
                .map(|v| convert_pixel(v, &conversion_map))
                .collect::<Vec<char>>(),
            vec![' ', ' ', '.', '+', '@']
        );
    }
}
