fn main() {
    // rectangle example with struct

    // we create a rectangle first]
    
    let rect1 =Rectanle{
        width:10,
        height:15,
    };

    // now we pass our object to main_area method and store the output
    let area1 = main_area(&rect1);

    println!("area of rect1 is {}" , area1);

    // side notes
    // what if we try to print rect1 instance
    // println!("rect1 is {}" , rect1); // here we will make an error
    // when we just use {} we are telling println to use 'dispaly formatting'
    // but our struct dont have this formatting
    // we use {:?} instead, which is called debug formatting
    // for that
    // 1. rewrite the println line like this
    println!("rect1 is: {:?}",rect1);
    // 2. add '#[derive(Debug)]' before rectangle struct deffnition


   
}

fn main_area(rect: &Rectanle) -> u32{ // we dont wnat the ownership of the parameter so we barrow it
    return rect.width*rect.height;
}

// our rectangle struct
#[derive(Debug)] // so far i think this line is something like extending or implementing another class
struct Rectanle{
    width: u32,
    height: u32,
}
