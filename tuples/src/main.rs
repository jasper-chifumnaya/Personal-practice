use std::io;

fn main() {
    let mut person: (&str, u32, f32) = ("", 0, 0.0);

    let mut name = String::new();
    let mut age = String::new();
    let mut height = String::new();

    let input = io::stdin();

    println!("What is your name?");
    input.read_line(&mut name).expect("Something went wrong!");
    let name = name.trim();

    person.0 = name;

    println!("What is your age?");
    input.read_line(&mut age).expect("Something went wrong!");
    let age: u32 = age.trim().parse().expect("Something went wrong!");

    person.1 = age;

    println!("What is your height?");
    input.read_line(&mut height).expect("Something went wrong!");
    let height: f32 = height.trim().parse().expect("Something went wrong!");

    person.2 = height;

    println!("{}", person.0);
}
