#![allow(warnings)]
use num::complex::Complex;
use std::collections::HashMap;

const WIDTH: i16 = 100;
const HEIGHT: i8 = 24;
const X_START: f64 = -2.0;
const X_END: f64 = 2.0;
const Y_END: f64 = -1.0;
const Y_START: f64 = 1.0;
const MAX_ITERS: isize = 1000;

fn create_canvas() -> Vec<Vec<(f64, f64)>> {
    // Creating x and y axis
    let mut axis: Vec<_> = Vec::new();
    for y in 0..HEIGHT {
        let mut row = Vec::new();
        for x in 0..WIDTH {
            let x_axis = X_START + (x as f64 * (X_END - X_START)) / (WIDTH - 1) as f64;
            let y_axis = Y_START + (y as f64 * (Y_END - Y_START)) / (HEIGHT - 1) as f64;
            row.push((x_axis, y_axis));
        }
        axis.push(row);
    }
    axis
}

fn escaped_at(co_ordinates: (f64, f64)) -> isize {
    // Escape algorithm
    let mut z = Complex { re: 0.00, im: 0.00 };
    let c = Complex {
        re: co_ordinates.0,
        im: co_ordinates.1,
    };
    for i in 0..=MAX_ITERS {
        if z.norm() > 2.0 {
            return i;
        }
        z = (z * z) + c;
    }
    MAX_ITERS
}

fn plot_manderblot(escaped_at_vec: Vec<isize>) {
    // For storing the sequence
    let mut str = String::new();
    for item in escaped_at_vec {
        // Transforming iter value into string sequence
        let val = match item {
            0..=2 => ' ',
            0..=5 => '.',
            5..=10 => '•',
            11..=30 => '*',
            30..=100 => '+',
            100..=200 => 'x',
            200..=400 => '$',
            400..700 => '#',
            _ => '%',
        };
        str.push(val);
    }
    println!("{}", str);
}

fn main() {
    println!("MandelBrot Set visualization in Rust");

    // Creating the x and y axis
    let axis = create_canvas();

    // Vector to store the no of iterations
    let mut iter_vec = Vec::new();
    for item in axis {
        for row_item in item {
            // The below function returns the iter value of the function when it blew up.
            let iter = escaped_at(row_item);

            // Storing the values into a vector
            iter_vec.push(iter);
        }
    }
    // The below vector shows the no. of iterations the equations took to blow up.
    println!("Escapted at vec : {:?}", iter_vec);

    // The below map shows the iter and count for visualization purposes.
    let mut map = HashMap::new();
    for val in &iter_vec {
        *map.entry(val).or_insert(0) += 1;
    }

    // Printing our map
    println!("{:?}", map);

    // Plotting the manderblot set
    plot_manderblot(iter_vec);
}
