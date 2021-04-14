use std::ops::Range;
use std::io::prelude::*;
use std::process::{Command, Stdio};

use rand::Rng;
use image::GenericImage;
use image::GenericImageView;
use image::io::Reader as ImageReader;
use image::Rgba;
use image::ImageBuffer;
use image::RgbImage;

struct Point
{
    pos: [u32; 2],
    color: Rgba<u8>,
}

impl Point
{
    fn dist(&self, coord: [u32; 2]) -> f64
    {
        let a: u32 = (self.pos[0] as i64 - coord[0] as i64).abs() as u32;
        let b: u32 = (self.pos[1] as i64 - coord[1] as i64).abs() as u32;
        ((a.pow(2) + b.pow(2)) as f64).sqrt()
    }
}

pub fn generate(width: u32, height: u32,
    point_count_range: Range<u32>, draw_points: bool, point_size: u32,
    red_range: &Range<u32>, green_range: &Range<u32>, blue_range: &Range<u32>,
    custom_color_generator: Option<String>,
    output_path: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let img: RgbImage = ImageBuffer::new(width, height);
    img.save(output_path)?;
    let mut img = ImageReader::open(output_path)?.decode()?;
    let (width, height) = img.dimensions();
    let point_count: u16 = rand::thread_rng().gen_range(point_count_range) as u16;
    if point_count == 0
    {
        return Ok(());
    }
    let mut points: Vec<Point> = Vec::with_capacity(usize::from(point_count));

    for _ in 0..point_count
    {
        let mut data: [u32; 5] = [0; 5];
        for i in 0..2
        {
            let dim = if i == 0 { width } else { height };
            data[i] = rand::thread_rng().gen_range(0..dim);
        }
        match custom_color_generator
        {
            None =>
            {
                for i in 2..5
                {
                    let color_range: &Range<u32> =
                    match i
                    {
                        2 => red_range,
                        3 => green_range,
                        4 => blue_range,
                        _ => &(0..256),
                    };
                    let color_range: Range<u32> = color_range.clone();
                    data[i] = rand::thread_rng().gen_range(color_range).clamp(0, 255);
                }
            },
            Some(ref cmd) =>
            {
                let process =
                match Command::new(cmd)
                                .stdout(Stdio::piped())
                                .spawn()
                {
                    Err(why) => panic!("couldn't spawn {}: {}", cmd, why),
                    Ok(process) => process,
                };

                let mut s = String::new();
                match process.stdout.unwrap().read_to_string(&mut s)
                {
                    Err(why) => panic!("couldn't read {} stdout: {}", cmd, why),
                    Ok(_) =>
                    {
                        let num_s_v: Vec<&str> = s.trim().split(" ").collect();
                        if num_s_v.len() != 3
                        {
                            panic!("incorrect number of outputs in {} stdout", cmd);
                        }
                        let mut num_v: Vec<u8> = Vec::new();
                        for num_s in num_s_v.iter()
                        {
                            num_v.push(num_s.parse()?);
                        }
                        for i in 2..5
                        {
                            data[i] = num_v[i - 2] as u32;
                        }
                    },
                }
            }
        }

        let pos: [u32; 2] = [data[0], data[1]];
        let color: Rgba<u8> = Rgba::from([data[2] as u8, data[3] as u8, data[4] as u8, 255]);
        let point: Point = Point { pos, color };

        points.push(point);
    }

    for x in 0..width
    {
        'height_loop: for y in 0..height
        {
            let mut closest_point: (&Point, f64);
            closest_point = (&points[0], points[0].dist([x, y]));
            if point_count > 1
            {
                for point in points.iter()
                {
                    let dist = point.dist([x, y]);
                    if draw_points && dist < point_size as f64
                    {
                        let color = Rgba::from([0u8, 0, 0, 255]);
                        img.put_pixel(x, y, color);
                        continue 'height_loop;
                    }
                    if dist < closest_point.1
                    {
                        closest_point = (point, dist);
                    }
                }
            }
            img.put_pixel(x, y, closest_point.0.color);
        }
    }

    img.save(output_path)?;

    Ok(())
}
