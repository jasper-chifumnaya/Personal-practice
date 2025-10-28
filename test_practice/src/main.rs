use std::io;

fn main() {

    let mut total = 0;

    //student name
    println!("Enter your name:");
    let mut stnam = String::new();
    io::stdin().read_line(&mut stnam).expect("Cannot read input, check if you entered a number");
    let stnam: String = stnam.trim().parse().expect("An error occurred");

    //scores
    println!("Enter your first score");
    let mut scone = String::new();
    io::stdin().read_line(&mut scone).expect("Wrong input");
    let scone: i32 = scone.trim().parse().expect("An error occured");

    total+=scone;

    println!("Enter your second score");
    let mut sctwo = String::new();
    io::stdin().read_line(&mut sctwo).expect("Wrong input");
    let sctwo: i32 = sctwo.trim().parse().expect("An error occured");

    total+=sctwo;

    println!("Enter your third score");
    let mut scthree = String::new();
    io::stdin().read_line(&mut scthree).expect("Wrong input");
    let scthree: i32 = scthree.trim().parse().expect("An error occured");

    total+=scthree;

    println!("Your total score is: {}", total);

    let avrg = total / 3;

    println!("Your average is: {}", avrg);

    if avrg <= 100 {
        if avrg >= 70 {
            println!("{} your grade is A", stnam);
        }
        else if avrg >= 60{
            println!("{} your grade is B", stnam);
        }
        else if avrg >= 50{
            println!("{} your grade is C", stnam);
        }
        else if avrg >= 45{
            println!("{} your grade is D", stnam);
        }
        else{
            println!("{} your grade is F", stnam);
        }
    }else {
        println!("Average is incorrect");
    }
}
