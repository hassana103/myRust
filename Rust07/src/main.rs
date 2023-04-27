fn main() {
    
    // rust ownership rules
    // 1. each value in Rust has an owner
    // 2. there can be only one owner at a time
    // 3. when the owner goes out of scope, the value will be dropped


    // ################################ scope

    // string literal
    // the value of string literals are hardcoded into the text of our program
    // s variable is not  valid here because its not declared yet
    let s = "my string"; // s is valid from this point forward
    // s is valid here, you can do stuff with s

}   // the scope is now over and s is no longer valid
