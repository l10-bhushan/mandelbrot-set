use num::complex::Complex;
const WIDTH: usize = 1000;
const HEIGHT: usize = 24;
const MAX_ITERATIONS: u16 = 1000;

#[derive(Debug)]
struct Canvas {
    view_port: Vec<Vec<f64>>,
}

impl Canvas {
    fn new() -> Canvas {
        Canvas {
            view_port: vec![vec![0.0; WIDTH]; HEIGHT],
        }
    }
}

fn create_axis_values(canvas: &mut Canvas) {
    let mid_x = (WIDTH / 2) as f64;
    let mid_y = (HEIGHT / 2) as f64;
    let step_size_x: f64 = (mid_x - (-mid_x)) / (WIDTH - 1) as f64;
    let step_size_y: f64 = (mid_y - (-mid_y)) / (HEIGHT - 1) as f64;
    println!("mid and step size : {mid_x} {mid_y} {step_size_x} {step_size_y} ");
    for i in 1..HEIGHT {
        for j in 1..WIDTH {
            let x_axis_val: f64 = (-mid_x + (j - 1) as f64 * step_size_x).round();
            let y_axis_val: f64 = (-mid_y + (i - 1) as f64 * step_size_y).round();
            let tuple = (x_axis_val, y_axis_val);
            let iters = distance(tuple);
            canvas.view_port[i - 1][j - 1] = iters;
        }
    }
}

fn distance(co_ordinates: (f64, f64)) -> f64 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let complex_number = Complex::new(co_ordinates.0, co_ordinates.1);
    for i in 0..=MAX_ITERATIONS {
        if z.norm() == 2.0 {
            return i as f64;
        }
        z = z * z + complex_number;
    }
    MAX_ITERATIONS as f64
}

fn main() {
    println!("---- Mandelbrot plot ----");

    let mut canvas = Canvas::new();
    create_axis_values(&mut canvas);
    println!("{:?}", canvas.view_port)
}
