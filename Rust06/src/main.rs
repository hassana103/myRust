fn main() {

    //Control Flow

    // #################################################################### IF
    let number = 5;

    if number < 10 {
        println!("first condition was tru");
    }else if number < 20 {
        println!("second condition was true");
    }else{
        println!("esle");
    }
    
    // use if inside let statment
    // somehow if returns a value !!!!
    let number2 = if number > 2 {"bigger"} else {"smaller"};



    // ###################################################################### loop
    loop {
        print!("inside loop");

        // to break the loop
        break;
    }

    // loop can return a value
    // add the value after the break
    // like this
    let mut i = 0;

    let res = loop {
        i +=1;
        if i == 10 {
            break i;
        }
    };

    // ###################################################################### while loop
    let mut j = 0;
    while j < 11 {
        println!("{}!" , j);

        j +=1;
    }


    //####################################################################### for loop
    let k = [10 , 20 ,30,40,50];
    for element in k.iter() {
        println!("{}" , element);
    }

    // for loop inside a range
    for s in 1..9 {

    }
    // 1..9 is a range
    //last number is exclusive 1..9 == [1......9)


    // single line comment

    /*
    multiline comments
     */



}
