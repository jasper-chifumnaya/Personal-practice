fn main() {

    let mut x: i32 = 13;

    let sq = square(x);

    println!("x^2 : {}", sq);

    // greet(&mut x);

    // let mut my_person: Person = Person::new(String::from("Uzochi"), 20);
    // Person::bark(); // associative function
    // my_person.call(); // method
}

// fn greet(name: &mut i32) {
//     *name = *name + 1;
//     println!("name: {}", name);
// }

fn square(num: i32) -> i32 {
    num * num
}

// struct Person {
//     name: String,
//     age: u32,
// }

// impl Person {
//     fn bark() {
//         println!("Woof!");
//     }

//     fn call(&self) {
//         println!("I am {} and I'm {} year(s) old.", self.name, self.age);
//     }

//     fn new(name: String, age: u32) -> Self {
//         Self {
//             name,
//             age,
//         }
//     }
// }