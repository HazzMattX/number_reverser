fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("must be a number");
    let input = input.trim();
    match input.parse::<i32>() {
        Ok(num) => {
            let reversed = if num < 0 {
                let digits: String = num.abs().to_string().chars().rev().collect();
                format!("-{}", digits)
            } else {
                num.to_string().chars().rev().collect()
            };
            match reversed.parse::<i32>() {
                Ok(reversed_num) => println!("Reversed number: {}", reversed_num),
                Err(_) => println!("Error: Could not convert back to a number"),
            }
        },
        Err(_) => println!("Invalid input! Use a valid number"),
    }
}
