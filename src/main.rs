use std::io::{self, Write};

use crate::services::calculator_service::Calculator;
mod services {
    pub mod calculator_service;
}
fn main() {
    println!(
        "please enter the calculate_type
    
    + => Addition
    - => Substraction
    / => Division
    * => Multiplication
     "
    );
    io::stdout().flush().unwrap();
    let mut calculate_type_input: String = String::new();
    io::stdin().read_line(&mut calculate_type_input).unwrap();

    println!("\nplease enter field1");
    let mut field1_input: String = String::new();
    io::stdin().read_line(&mut field1_input).unwrap();

    let field1: f32 = match field1_input.trim().parse() {
        Ok(result) => result,
        Err(_) => {
            print!("error input");
            return;
        }
    };
    println!("your field1 data is {}", field1);

    println!("\nplease enter field2");
    let mut field2_input: String = String::new();
    io::stdin().read_line(&mut field2_input).unwrap();

    let field2: f32 = match field2_input.trim().parse() {
        Ok(result) => result,
        Err(_) => {
            print!("error input");
            return;
        }
    };
    println!("your field1 data is {}", field2);
    let calcutor = match Calculator::new(field1, field2, calculate_type_input.trim()) {
        Ok(new_calculator) => new_calculator,
        Err(_) => {
            println!("error");
            return;
        }
    };
    println!(
        "calculated data => {} {} {} = {}",
        calcutor.field1,
        calcutor.find_calculate_type_as_string(),
        calcutor.field2,
        calcutor.calculate()
    )
    //let calculate_type: &str = calculate_type_input.trim();
}
