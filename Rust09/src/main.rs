fn main() {
    
    let a = 10;
    // just like before, if i use a as a parameter to call a method
    // the method wont take ownership of a and the value of a is copied
    // for example
    makes_copy(a);

    // because the method does not take the ownership
    // we can use a after using it as a parameter
    // for example
    println!("a is still valid here: {}" , a);


    // but what about complex types
    let s = String::from("hello");
    // what happens when we pass a complex variable as a parameter
    takes_ownership(s);
    // also in this situation , just like before, because the parameter has 
    // a complex type, the value will not simply just copy
    // and the ownership of s is moved to the method(to the parameter to be more specific)
    // and after completion of the method, the scope of parameter ends and because of that
    // the value will be droped and no longer it is usable
    // so we cant use s here
    // println!("this line wont compile: {}" , s); 


}

fn makes_copy(i:i32){
    // this method wont take ownership of its parameter
    // bacause the parameter has a simple type the value will be copied to i
    println!("{}" , i);
}


fn takes_ownership(s:String){
    // as the parameter has a complex type, this method will take its ownership
    println!("{}" , s);
}
