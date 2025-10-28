use std::io;

fn main(){
	
	loop {
		
		println!("Do you want to calculate a Compound Interest? (Type Yes or No)");
		let mut user_reply = String::new();
		io::stdin().read_line(&mut user_reply).expect("You entered a wrong value");
		let user_reply: String = user_reply.trim().parse().expect("Enter a reply");

		if user_reply == "yes" {
		
			println!("Enter principle:");
		    let mut prin = String::new();
		    io::stdin().read_line(&mut prin).expect("Failed to read input");
		    let prin:f32 = prin.trim().parse().expect("Failed to read input");

		    println!("Enter rate:");
		    let mut rate = String::new();
		    io::stdin().read_line(&mut rate).expect("Failed to read input");
		    let rate:f32 = rate.trim().parse().expect("Failed to read input");

			println!("Enter time(in years):");
		    let mut time = String::new();
		    io::stdin().read_line(&mut time).expect("Failed to read input");
		    let time:f32 = time.trim().parse().expect("Failed to read input");

		    let amount : f32 = prin * (1.0 + (rate/100.0)).powf(time);

		    println!("The amount is {}", amount);
		}else if user_reply == "no" {
			break;
		}else {
			println!("Input is invalid, try again");
			continue;
		}

	}

}