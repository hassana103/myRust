fn main() {
    
    let s = String::from("hello");

    // 1. you have to annotate the type of i here -> means we should define i as i:i32
    // 2. the result of parse method is 2 things, its either ok or an error
    // so you can use match to handle this 2 situations
    let i:i32 =  match s.parse() {
        // if the result of parse is ok return the value
        Ok(num) => num,
        // if an error happend (_ means any error) just print an error and return
        Err(_) => {
            println!("Err: not a number");
            return;
        }
    };

    println!("this is your number: {}" , i);
}