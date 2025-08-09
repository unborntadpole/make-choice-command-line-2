use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::rng();
    let mut input = String::new();
    println!("Enter number of choices:");
    let num;
    loop{
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<usize>(){
            Ok(n) => {
                num = n;
                break;
            },
            Err(error) => println!("Failed to read input, please enter valid number: {error}"),
        }
        input.clear();
    }
    let limit;
    println!("Now tell me how many times in a row do you want me to get a particular choice before I choose it as the final one:");
    loop{
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<usize>(){
            Ok(n) => {
                limit = n;
                break;
            },
            Err(error) => println!("Failed to read input, please enter valid number: {error}"),
        }
    }
    
    println!("Now enter your choices manually. You have chosen to have {} choices.", num);
    let mut choices: Vec<String> = Vec::new();
    for i in 1..=num {
        input.clear();
        println!("Enter choice {}:", i);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        choices.push(input.trim().to_string());
    }
    let mut last_choice: Option<&str> = None;
    let mut same_choice_counter = 0;
    let mut counter = 0;
    loop {
        println!("\nNow moving on to making a choice for you.");
        let choice: usize = rng.random_range(0..num);
        let choice: &str = choices[choice].as_str();
        println!("I choose.. {}", choice);
        if let Some(s) = last_choice {
            if s == choice {
                same_choice_counter += 1;
            } else {
                same_choice_counter = 1;
                last_choice = Some(choice);
            }
        } else {
            same_choice_counter = 1;
            last_choice = Some(choice);
        }
        let s = match same_choice_counter%10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        };
        println!("{}{} occurance of choice {}", same_choice_counter, s, choice);
        counter += 1;
        if same_choice_counter == limit {
            let num = num as f64;
            let limit = limit as f64;
            println!(
                "\n\nGod has chosen the same option {} times.\nThe expectation of this hapenning was {} turns.\nYou are gonna have to go with {} as your choice.",
                limit,
                (num.powf(limit)-1.0)/(num -1.0),
                choice,
            );
            println!("Phew! It took {} choices to come to this decision.", counter);
            break;
        }
        println!("{} more time{} and I will choose this as the final result...", limit-same_choice_counter, if limit-same_choice_counter == 1 {""} else {"s"});

    }
}
