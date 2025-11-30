fn main() {

    let x = 5;
    let mut y = 3; // Needed 'mut' because we change its value later
    let y = calculate(x, y);
    let z = x + y;

    println!("{:?}", x);
    println!("{}", y);
    println!("{}", z);
}

fn calculate(mut a: i32, mut b: i32) -> i32{
    a = a + 1;
    b = a * 2;
    return b;
}