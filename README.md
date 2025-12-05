# 🌀 Mandelbrot Set Visualizer

The **Mandelbrot Set** is one of the most famous fractals in mathematics.  
It represents the set of complex numbers **c** for which the function:

f(z) = z² + c

does **not diverge** when iterated from `z = 0`.

This project generates and visualizes the Mandelbrot Set using iterative computation and a string sequence based on escape time.

---

## 📌 What Is the Mandelbrot Set?

A point `c = a + bi` (complex number) belongs to the Mandelbrot set if the sequence:

z₀ = 0
zₙ₊₁ = zₙ² + c

remains bounded (doesn't go to infinity) no matter how many times it is repeated.

A point is considered **outside the set** if `|zₙ| > 2` during iteration.

---

## 🧠 Algorithm (Escape-Time Method)

1. For each pixel, map it to a point in the complex plane.
2. Initialize:
3. Repeat up to `max_iterations`:
4. 4. If `|z| > 2`, the point escapes.
5. If the loop completes without escape, the point is inside the set (usually colored black).

Example plotting formula:

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

## 🖥 Output Preview

Below is the Mandelbrot Set rendered in the terminal using ASCII:

<img width="1110" height="708" alt="Screenshot 2025-12-05 at 8 22 55 PM" src="https://github.com/user-attachments/assets/2b5998ab-8e79-43ba-86ff-fe2365263e40" />
