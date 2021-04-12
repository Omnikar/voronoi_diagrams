use std::ops::Range;

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

pub fn generate(width: u32, height: u32, point_count_range: Range<u32>, draw_points: bool, point_size: u32, output_path: &str) -> Result<(), Box<dyn std::error::Error>>
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
        for i in 2..5
        {
            data[i] = rand::thread_rng().gen_range(0..256);
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
