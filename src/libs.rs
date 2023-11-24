use std::{io, num::ParseIntError};

pub fn int_input(question: &str) -> i32 {
    println!("{}", question);
    
    let mut user_res = String::new();
    io::stdin().read_line(&mut user_res).expect("Line read error.");
    let user_res: Result<i32, ParseIntError> = user_res.trim().parse();

    match user_res {
        Ok(res) => {
            return res;
        }
        _ => {
            let err_code = 0;
            return err_code;
        }
    }
}

pub fn str_input(question: &str) -> String {
    println!("{}", question);
    
    let mut user_res = String::new();
    io::stdin().read_line(&mut user_res).expect("Line read error.");
    let user_res = user_res.trim().to_string();
    return user_res;
}