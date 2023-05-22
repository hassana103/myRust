fn main() {

    // struct -> just like a class in object oriented progamming languages
    // grouping related data
    // they can have different names
    
    // to use a struct we should create instances of that struct
    let user1 = User{
        // you dont have to give the values in the same order you defined the struct
        email: String::from("hassan@cyt.com"),
        name: String::from("Leon"),
        is_active: true,
        login_count: 1
        
    }; // dont forget this ;


    // if you want to access the value of a specific field you sould use dot notation
    let name = user1.name;
    println!("this is the first users name: {}" , name);

    // if you want to be able to change fields after instantiating
    // you sould define that instance as mut
    // so we cant change the name of user1 here
    // user1.name=String::from("new name"); //this line wont compile
    let mut user2=User {
        name: String::from("Bob"),
        email: String::from("bob@cyt.ir"),
        is_active: true,
        login_count: 10,
    };
    // now we can change the fields
    user2.name = String::from("John");
    println!("the changed name of user2 is :{}",user2.name);


}

// to define a new struct we use 'struct' keyword then curly brackets
struct User
{
    // this are fields
    name: String,
    email:String,
    is_active: bool,
    login_count: u32,
}
