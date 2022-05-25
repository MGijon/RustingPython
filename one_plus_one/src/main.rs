//use rand::Rng;
//use rand_distr::Normal;
use rand::Rng;

// -------
// REF: https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable
pub trait AnyExt{
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T{
    fn type_name(&self) -> &'static str{
        std::any::type_name::<T>()
    }
}
// -------

fn sphere(x: f64, y: f64) -> f64{
    // 2-dimensional computation of the sphere function
    x*x + y*y
}

fn update_value(original_value: f64) -> f64{
    // add a perturbation to the given value
    let mut rng = rand::thread_rng();
    let perturbation = rng.gen::<f64>();
    let coin = rng.gen::<f64>();
    let b: f64;
    if coin >= 0.5{
        b = original_value - perturbation;
    } else {
        b = original_value + perturbation;
    }
    b
}

fn main() {
    // initial points
    let mut x: f64 = 23.93990;
    let mut y: f64 = 7.870;

    let mut initial_value = sphere(x, y);
    println!("Initial spehre value: {}", initial_value);
    let number_steps: usize = 10000;

    for i in 0..number_steps{
        let new_x = update_value(x);
        let new_y = update_value(y);

        let new_sphere_value: f64 = sphere(new_x, new_y);
        if new_sphere_value < initial_value{
            // update sphere benchmark
            initial_value = new_sphere_value;
            x = new_x;
            y = new_y;
        }
        if i % 1000 == 0{
            println!("Iteration: {} - Value: {}", i, initial_value);
        }
    }
    println!("{} {}: {}", x, y, initial_value);
}

