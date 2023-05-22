// use std::fs::File;
pub fn error_out() {
    // println!("Hello, World!");

    // // Explicitly exit the program with an unrecoverable error
    // // panic!("Crash");

    // let data_result = File::open("data.txt");

    // // using match for Result type
    // let data_file = match data_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the data file: {:?}", error),
    // };

    // println!("Data filen {:?}", data_file);


    let text = "Hello, World!";
    
    let character_option = text.chars().nth(15);
    
    // using match for Option type
    let character: String = match character_option {
        None => "does not exist".to_string(),
        Some(c) => c.to_string()
    };
    
    println!("Character at index 15 is {}", character);

}


pub fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }

    return Some(username);
}

pub fn calculate_length(s: &String) -> usize {
    s.len()
}