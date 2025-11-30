struct Student {
    name: String,
    age: u32,
    matric: String,
    dept: String,
}

impl Student {
    fn shout() {
        println!("(*Screams*)");
    }

    fn describe(&self) {
        println!("My name is {} and I am in {} department!", self.name, self.dept);
    }
}

fn main() {
    let my_student: Student = Student {
        name: String::from("Uzochi"),
        age: 20,
        matric: String::from("1234567"),
        dept: String::from("Software Eng.")
    };

    my_student.describe();
}
