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

    let point_size: u32 = config.get("generator", "point_size").unwrap().parse()?;
    let draw_points: bool = config.get("generator", "draw_points").unwrap().parse()?;

    let output_path: String = config.get("generator", "output_path").unwrap();

    println!("Generating Voronoi diagram...");

    voronoi::generate(
        width, height,
        point_count_min..point_count_max,
        draw_points, point_size,
        &output_path)?;

    println!("Done!");

    Ok(())
}
