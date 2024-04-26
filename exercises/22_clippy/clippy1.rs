// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy warnings
// check clippy's suggestions from the output to solve the exercise.
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a hint.

// Utilisation directe de la constante PI de std::f32::consts au lieu d'une valeur littérale pour pi
use std::f32::consts::PI;

fn main() {
    let radius = 5.00f32;
    // Calcul de l'aire en utilisant directement la constante PI
    let area = PI * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
