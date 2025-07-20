const SCALE_FACTOR: u64 = 150;

const X_SCALE: std::ops::Range<f64> = -2.00..0.47;
const Y_SCALE: std::ops::Range<f64> = -1.12..1.12;
const X_PAN: f64 = -1.5;
const Y_PAN: f64 = -0.5;
const VIEWPORT: u64 = 150;
const MAX_ITERATION: usize = 1000;
const SYMBOL_ARRAY: [&str; 5] = [" # ", " * ", " - ", " . ", "   "];

#[derive(Clone, Copy, Debug)]
struct Pixel(f64, f64);

impl Pixel {
    fn new(coords: (u64, u64)) -> Self {
        let x_increment = (X_SCALE.end - X_SCALE.start) / SCALE_FACTOR as f64;
        let y_increment = (Y_SCALE.end - Y_SCALE.start) / SCALE_FACTOR as f64;
        Self(
            X_PAN + (coords.0 as f64 * x_increment),
            Y_PAN + (coords.1 as f64 * y_increment),
        )
    }
}

fn main() {
    (0..=VIEWPORT)
        .flat_map(|y_coord| {
            println!();
            (0..=VIEWPORT).map(move |x_coord| (x_coord, y_coord))
        })
        .for_each(|coords| {
            let pixel = Pixel::new(coords);

            let current_symbol = mandelbrot_pixel(pixel) * (SYMBOL_ARRAY.len()
                - 1 ) /* Fencepost Error */ / MAX_ITERATION;
            print!("{}", SYMBOL_ARRAY[current_symbol]);
        });
}

fn mandelbrot_pixel(pixel: Pixel) -> usize {
    let value = (0..=MAX_ITERATION).fold((0.0, 0.0, 0), |acc, iteration| {
        let x: f64 = acc.0;
        let y: f64 = acc.1;
        if x.powf(2.0) + y.powf(2.0) > 4.0 {
            acc
        } else {
            let next_x = x.powf(2.0) - y.powf(2.0) + pixel.0;
            let next_y = 2.0 * x * y + pixel.1;
            (next_x, next_y, iteration)
        }
    });

    value.2
}
