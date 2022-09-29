use std::io;

fn main() {
    let mut operation = String::new();
    let mut first_num = String::new();
    let mut second_num = String::new();
    let stdin = io::stdin(); 
    println!("Input Operation");
    stdin.read_line(&mut operation);        
    
    println!("Input First Number");
    stdin.read_line(&mut first_num);       
    let my_first_num: i32 = first_num.trim().parse()
      .expect("please give me correct string number!");

    println!("Input Second Number");
    stdin.read_line(&mut second_num);
    let my_second_num: i32 = second_num.trim().parse()
      .expect("please give me correct string number!");

    print!("{} {} {} ", my_first_num , operation.trim() , my_second_num );
    match operation.as_str().trim() {
        "*" => print!(" = {}", my_first_num * my_second_num),
        "/" => print!(" = {:.}", (my_first_num as f64) / (my_second_num as f64)),
        "+" => print!(" = {}", my_first_num + my_second_num),
        "-" => print!(" = {}", my_first_num - my_second_num),
        _ => {
          println!("is an Invalid Operation");
          return;
        }
    }
    return;  
}