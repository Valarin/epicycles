use std::{fs, io};
use std::ops::Add;
use std::path::{Path, PathBuf};

use euclid::{Angle, Point2D, Vector2D};
use image::{ImageBuffer, RgbImage};
use serde::{de, Deserialize, Deserializer};
use structopt::StructOpt;

const SAMPLES_PER_SECOND: f32 = 500.0;
const CANVAS_SIZE: u32 = 1000;
const SAMPLE_COUNT: u32 = 10000;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(short, long, parse(from_os_str))]
    pub input: PathBuf,
    #[structopt(short, long)]
    pub output: String,
}

#[derive(Debug, Deserialize)]
struct Epicycle {
    angular_speed_rad_s: f32,
    radius: f32,
    #[serde(deserialize_with = "from_f32")]
    initial_angle: Angle<f32>,
}

fn from_f32<'de, D>(deserializer: D) -> Result<Angle<f32>, D::Error> where D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Angle::radians).map_err(de::Error::custom)
}

fn main() {
    let args = Args::from_args();

    let epicycles = read_input(&args.input);
    let (pixels, bottom_left, upper_right) = sample_epicycles_in_time(epicycles);
    print_picture(pixels, bottom_left, upper_right, args.output);
}

fn read_input(input_path: &Path) -> Vec<Epicycle> {
    let input_file = fs::File::open(input_path).expect("Could not open file");
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::BufReader::new(input_file));

    let mut epicycles = Vec::new();
    for result in csv_reader.deserialize() {
        let epicycle: Epicycle = result.expect("Failed to read record");

        epicycles.push(epicycle)
    }

    epicycles
}

fn sample_epicycles_in_time(epicycles: Vec<Epicycle>) -> (Vec<Point2D<i32, f32>>, Point2D<i32, f32>, Point2D<i32, f32>) {
    let mut pixels = Vec::new();
    let mut bottom_left_x = i32::MAX;
    let mut bottom_left_y = i32::MAX;
    let mut upper_right_x = i32::MIN;
    let mut upper_right_y = i32::MIN;

    for time in 1..SAMPLE_COUNT {
        let mut sampled_vector = Vector2D::zero();

        for epicycle in &epicycles {
            let angle_change_in_time = Angle::radians((epicycle.angular_speed_rad_s * time as f32) / SAMPLES_PER_SECOND);
            let angle_now = epicycle.initial_angle.add(angle_change_in_time);

            let vector: Vector2D<f32, f32> = Vector2D::from_angle_and_length(angle_now, epicycle.radius);

            sampled_vector = sampled_vector.add(vector);
        }

        let sampled_point = sampled_vector.to_i32().to_point();

        bottom_left_x = bottom_left_x.min(sampled_point.x);
        bottom_left_y = bottom_left_y.min(sampled_point.y);
        upper_right_x = upper_right_x.max(sampled_point.x);
        upper_right_y = upper_right_y.max(sampled_point.y);

        pixels.push(sampled_point);
    }

    (pixels, Point2D::new(bottom_left_x, bottom_left_y), Point2D::new(upper_right_x, upper_right_y))
}


fn print_picture(pixels: Vec<Point2D<i32, f32>>, min: Point2D<i32, f32>, max: Point2D<i32, f32>, output: String) {
    let mut image: RgbImage = ImageBuffer::new(CANVAS_SIZE, CANVAS_SIZE);

    let normalization_vector: Vector2D<i32, f32> = Vector2D::new(-min.x, -min.y);
    let scale_factor = (((max.x - min.x) as u32).max((max.y - min.y) as u32) / CANVAS_SIZE) + 1;

    for pixel in pixels {
        let moved_pixel: Point2D<u32, f32> = pixel.add(normalization_vector).to_u32();
        let scaled_pixel: Point2D<u32, f32> = Point2D::new(moved_pixel.x / scale_factor, moved_pixel.y / scale_factor);

        let pixel = image.get_pixel_mut(scaled_pixel.x, scaled_pixel.y);
        *pixel = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
    }

    image.save(output).expect("Failed to store image");
}
