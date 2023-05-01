fn main() {
    // simple types : they have a known size -> they can be stored on the stack
    // for example i32(integer) has a fix size and we know the size at compile time
    // simple types can be copied to make a new independent instance
    // for example
    let a = 10;
    let b = a ; // in this case the value of a (10) is copied and b is a new variable
                    // of type i32 and completly seperate from a   

    // string literals are stored directly in the program text

    // to explain ownership we need a complex type because they are stored on the heap
    // and their size are not knowen at compile time
    // something like String type
    // String type is different from string literals
    // you can creat a string from a string literal
    let s = String::from("hello");


    // multiple variables can intract with the same data in different ways
    // for example
    let a1 = 12 ;
    let b1 = a;
    // as explained in line 7, in this case because i32 is a simple type with fised size
    // the data is copied by default and the two variables a and b are completly independent
    // so you can work with each one freely
    println!("this is a1: {}" , a1);
    println!("this is a2: {}" , b1);

    // but when we work with complex types it is different 
    // for example
    let s1 = String::from("My String"); // a memory allocation is accoured here for s1
    let s2 = s1;
    // in this situation the value is not copied and a new memory allocation is not accoured for s2
    // in other hand just the ownership of our value is moved from s1 to s2
    // and if you remember the second ownership rule in rust you can guess that s1 is no longer usable
    // because there should be only one owner for a value at a time
    // and s2 is now the owner of this value
    // to be sure you can uncomment next line and run the program you will get an error
    // println!("im trying to use s1 : {}" , s1);
    


}
