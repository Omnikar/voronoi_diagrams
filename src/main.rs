extern crate image;

mod voronoi;

use std::io;
use configparser::ini::Ini;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    println!("Enter path of config file:");

    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)?;
    path = String::from(path.trim());

    let mut config = Ini::new();
    config.load(&path)?;

    let width: u32 = config.get("generator", "width").unwrap().parse()?;
    let height: u32 = config.get("generator", "height").unwrap().parse()?;

    let point_count_min: u32 = config.get("generator", "point_count_min").unwrap().parse()?;
    let point_count_max: u32 = config.get("generator", "point_count_max").unwrap().parse()?;

    let draw_points: bool = config.get("generator", "draw_points").unwrap().parse()?;
    let point_size: u32 = config.get("generator", "point_size").unwrap().parse()?;

    let output_path: String = config.get("generator", "output_path").unwrap();

    let red_min: u32;
    match config.get("generator", "red_min")
    {
        Some(s) => red_min = s.parse::<u32>()?.clamp(0, 255),
        None => red_min = 0,
    }
    let red_max: u32;
    match config.get("generator", "red_max")
    {
        Some(s) => red_max = s.parse::<u32>()?.clamp(red_min + 1, 256),
        None => red_max = 256,
    }

    let green_min: u32;
    match config.get("generator", "green_min")
    {
        Some(s) => green_min = s.parse::<u32>()?.clamp(0, 255),
        None => green_min = 0,
    }
    let green_max: u32;
    match config.get("generator", "green_max")
    {
        Some(s) => green_max = s.parse::<u32>()?.clamp(green_min + 1, 256),
        None => green_max = 256,
    }

    let blue_min: u32;
    match config.get("generator", "blue_min")
    {
        Some(s) => blue_min = s.parse::<u32>()?.clamp(0, 255),
        None => blue_min = 0,
    }
    let blue_max: u32;
    match config.get("generator", "blue_max")
    {
        Some(s) => blue_max = s.parse::<u32>()?.clamp(blue_min + 1, 256),
        None => blue_max = 256,
    }

    let custom_color_generator = config.get("generator", "custom_color_generator");

    println!("Generating Voronoi diagram...");

    voronoi::generate(
        width, height,
        point_count_min..point_count_max,
        draw_points, point_size,
        &(red_min..red_max), &(green_min..green_max), &(blue_min..blue_max),
        custom_color_generator,
        &output_path)?;

    println!("Done!");

    Ok(())
}
