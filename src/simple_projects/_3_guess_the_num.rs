use rand::random;
use std::io::{stdin, stdout, Write};
const MAX_GUESS_COUNT: u8 = 5;

fn count_with_suffix<T>(_count: T) -> String
where
    T: num::Unsigned + num::PrimInt,
{
    let count = _count.to_usize().unwrap();
    match count % 100 {
        11 | 12 | 13 => format!("{}th", count), // Handle 11, 12, 13 as exceptions
        _ => match count % 10 {
            1 => format!("{}st", count),
            2 => format!("{}nd", count),
            3 => format!("{}rd", count),
            _ => format!("{}th", count),
        },
    }
}

fn ask_user(actual: u8, guess_count: u8) -> bool {
    print!("{} Guess: ", count_with_suffix(guess_count));
    stdout().flush().unwrap();
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let mut result = false;
    match user_input.trim().parse::<i8>() {
        Ok(guess) if (1..=20).contains(&guess) => {
            if guess as u8 == actual {
                println!("Your guess is absolutely correct !!");
                result = true;
            } else if guess_count > MAX_GUESS_COUNT {
                println!("Your guess is incorrect !")
            } else {
                println!("Your guess is incorrect ! Try again.")
            }
        }
        Ok(_) => println!("Please enter a number >=1 and <=20"),

        Err(_) => println!("Please enter a valid number."),
    }

    result
}
pub fn guess_the_num() {
    let rnd: u8 = random();
    let actual: u8 = (rnd % 20) + 1;
    println!("Guess the number.\nHint: The number is >=1 and <=20.");
    let game_won = (1..=MAX_GUESS_COUNT).any(|guess_count| ask_user(actual, guess_count));
    // for count in 1..=MAX_GUESS_COUNT {
    //     if ask_user(actual, count) {
    //         break;
    //     };
    // }
    if !game_won {
        print!("The number was {}. ", actual);
    }
    println!("GAME OVER")
}
