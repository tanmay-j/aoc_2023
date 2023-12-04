use std::io;

mod first;

fn main() {
    println!("Which day do you want solved?");

    let mut day = String::new();

    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");

    let day:u32 = day.trim().parse().expect("Not a valid number!");

    println!("You chose day {day}");

    match day{
        1=>first::run(),
        _=>println!("Unsolved"),
    }
}
