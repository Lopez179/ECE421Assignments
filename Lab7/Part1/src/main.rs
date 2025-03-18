use std::cmp::max;
use std::io;
unsafe extern "C" {
    pub fn abs(i: i32) -> i32;
}

pub struct Point {
    x: i32,
    y: i32,
}
pub fn compute_euclidean_distance(p1: &Point, p2: &Point) -> f64 {
    let x1 = p1.x as f64;
    let x2 = p2.x as f64;
    let y1 = p1.y as f64;
    let y2 = p2.y as f64;
    ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).powf(0.5)
}
pub fn compute_manhattan_distance_c(p1: &Point, p2: &Point) -> i32 {
    unsafe
    {
        let a_abs = abs(p2.x as i32 - p1.x as i32);
        let b_abs = abs(p2.y as i32 - p1.y as i32);
        a_abs + b_abs
    }
}

pub fn compute_chebyshev_distance(p1: &Point, p2: &Point) -> i32 {
    max((p1.x - p2.x).abs(), (p1.y - p2.y).abs())
}

pub fn compute_chebyshev_distance_c(p1: &Point, p2: &Point) -> i32 {
    unsafe
    {
        let x_abs = abs(p2.x as i32 - p1.x as i32);
        let y_abs = abs(p2.y as i32 - p1.y as i32);
        max(x_abs, y_abs)
    }
}

fn prompt_int_input(msg: &str) -> i32{
    println!("{}", msg);
    let mut output = 0;

    let mut accepted = false;
    
    while !accepted {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();
        match input.parse::<i32>() {
            Ok(num) => {
                output = num;
                accepted=true
            },
            Err(e) => {
                println!("Invalid Input: {}", e)
            },
        }
    }
    output
}

fn full_test_loop() {
    let mut exit = false;
    while !exit {

        println!("What would you like to do:");
        println!("0. Exit");
        println!("1. Euclidean Distance");
        println!("2. Manhattan Distance");
        println!("3. Chebyshev Distance");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();
        match input {
            "0" => {
                exit = true
            },
            "1" => {
                let (p1, p2) = create_points_from_inputs();
                println!("Euclidean Distance: {}",compute_euclidean_distance(&p1, &p2));
            },
            "2" => {
                let (p1, p2) = create_points_from_inputs();
                println!("Manhattan Distance: {}",compute_manhattan_distance_c(&p1, &p2));
            },
            "3" => {
                let (p1, p2) = create_points_from_inputs();
                println!("Chebyshev Distance: {}",compute_chebyshev_distance_c(&p1, &p2));
            },
            _ => {
                println!("Please put a valid input")
            },
        }
    }
}

fn create_points_from_inputs() -> (Point, Point) {
    let x1 = prompt_int_input("Point 1 x:");
    let y1 = prompt_int_input("Point 1 y:");
    let x2 = prompt_int_input("Point 2 x:");
    let y2 = prompt_int_input("Point 2 y:");
    let p1 = Point {x: x1, y: y1};
    let p2 = Point {x: x2, y: y2};
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_addition() {
        let p1 = Point {x: 5, y: 7};
        let p2 = Point {x: 8, y: 11};
        assert_eq!(compute_euclidean_distance(&p1, &p2), 5.0);
    }
}

fn main() {
    full_test_loop();
}
