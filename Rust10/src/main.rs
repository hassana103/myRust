fn main() {
    
    // as mentioned in last project(Rust09), methods can take the ownership of a variable
    // but what if we want the variable back
    // in this situation we can have the method return the value

    let s = String::from("hello");

    // now we call the method and give the ownership of s to it
    // but because the method returns the value now we can have the value back
    let s2 = take_ownership_and_gives_back(s);

    // ofcorse we cant use s here
    // atleast we can have its value back as s2
}

fn take_ownership_and_gives_back(p:String)->String {
    println!("this is the parameter: {}" , p);
    return p;
}
