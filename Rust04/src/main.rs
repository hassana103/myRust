fn main() {
    println!("Hello, world!");

    // prove that variables are immutable
    //first we create a variable
    let x = 5;
    println!("the value of x is {}" , x);

    //now we try to change its value
    // next line will not compile
    // and you get an error because you tried to change an immutable varible
    // x=6;
    println!("the value of x is {}" , x);


    // but if you want to define a mutable varible
    // you should use 'mut' keyword
    let mut y =5;

    // now because y is mutable you can change its value
    y=6;
    println!("the value of y is {}" , y);

    //#######################################################################################################
    // constant values - values that never can change
    const CONST_VALUE:u32 = 100000;

    // Q: why do we need constants when the variable are immutable by default
    // A:   1. you cannot mutate a constant variable
    //      for example you cannot say 'const mut CONST_VAR'
    //      2. const variable most be type annotated - u32 for example
    //      3. the value of const variables must be declared at compile time
    //      then you cant set the return value of a function as the value


    //#######################################################################################################
    // scalar data types vs compound data type
    
    // rust has 4 main scalar types
        // Integers
        // Floating-point numbers
        // Booleans
        // Character


        // ################################## Integers 
        //  Length  signed  unsigned
        //  8-bit   i8      u8
        //  16-bit  i16     u16
        //  32-bit  i32     u32
        //  64-bit  i64     u64
        //  128-bit i128    u128
        //  arch    isize   usize

        let a= 98_765; // Decimal
        let b = 0xab; // Hex
        let c = 0o77; // Octal
        let d = 0b10101; //Binary
        let e = b'A'; // Byte


        // ################################## Floating Point
        let f = 2.3;
        let g: f32 = 3.4;

        let sum = 2.3 + 3.7;
        let dif = 95.4 - 32.7;
        let prod = 3.2 * 5.4;
        let div = 43.8 / 32.1;
        let rem = 43 % 3;


        // ##################################  Booleans
        let t = true;
        let f = false;


        // ################################# Character
        let c = 'z';
        let z = '@';




        // ###################################################################################################
        // Compound types - types that represent a group of values

        // ###################################### tuple
        // a fixed sized array of related data that can have diffrent types
        let tup = ("Hello tuple" , 123 , 'A');

        //how to access tuple values
        let (a,b,c) = tup;
        println!("{}",a);

        //or
        println!("{}" , tup.0);

        // if the tuple was mutable you could change its values like this
        // tup.0 = "another text";


        // ###################################### array
        // fixed size
        let array1 = [100,200,300];
        let a1 = array1[1];

        // create an array with 8 values all set to 0
        let array2 = [0; 8];



}
