use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;

use crate::custom_types;

pub fn plot_path_evolution(path: Vec<custom_types::custom_types::Point>) -> Result<(), Box<dyn std::error::Error>>{
    //path: &Vec<basic::Point>) -> bool{
    // Data preparation
    let number_of_points: usize = path.len();
    //println!("{}", number_of_points);

    // This part is unneessary since is not in use
    let mut xs: Vec<f64> = vec![];
    let mut ys: Vec<f64> = vec![];

    for i in 0..number_of_points {
        xs.push(path[i].x);
        ys.push(path[i].y);
    }

    //let max_x: f64 = *xs.iter().max().unwrap();
    //let min_x: f64 = *xs.iter().min().unwrap();
    //let max_y: f64 = *ys.iter().max().unwrap();
    //let min_y: f64 = *ys.iter().min().unwrap();
    let mut max_x: f64 = xs[0];
    let mut max_y: f64 = xs[0];
    let mut min_x: f64 = ys[0];
    let mut min_y: f64 = ys[0];
    for i in 0..number_of_points{
        // max
        if xs[i] > max_x{
            max_x = xs[i];
        }
        if ys[i] > max_y{
            max_y = ys[i];
        }
        // min
        if xs[i] < min_x{
            min_x = xs[i];
        }
        if ys[i] < min_y{
            min_y = ys[i];
        }
    }


    // graphic shit
    let root = BitMapBackend::new("graphic.png", (640, 480)).into_drawing_area();
    root.fill(&RGBColor(255, 255, 255))?;

    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf64, RangedCoordf64>::new(
        min_x..max_x,
        min_y..max_y,
        //-10f64..10f64,
        //-10f64..10f64,
        (0..640, 0..480),
    ));


    let dot_and_label = |x: f64, y: f64, z: f64| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
            + Text::new(
                format!("{:.2}", z),
                (10, 0),
                ("sans-serif", 15.0).into_font(),
            );
    };

    for i in 0..number_of_points{
        root.draw(&dot_and_label(path[i].x, path[i].y, path[i].z))?;
    }

    Ok(())
}
