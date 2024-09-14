

use rand;
use std::env;
fn main() {

    let ran_numbers = rand::random::<f64>();
    //COPY AND MOVE ON DIFF DATA TYPES
    //Copying data on the stack occures implictly while copying data on the heap must be done explicitly, using the clone method instead.
    // Data stored on the Heap ==> Sring
    let outer_planet : String;
    {
        let mut inner_planet = String::from("Mercury");
        //outer_planet = inner_planet; In Rust This is a move 
        //code wont complie also.
        outer_planet = inner_planet.clone();
        inner_planet.clear();
        println!("inner planet: {}", inner_planet);


    }

    println!("outer planet: {}", outer_planet);



    //Data stored on the stack
    let outer_number : i32; // Leaves completely on on the stack and doesn't reference anything on the heap

    {
       let mut inner_number = 5;
        outer_number = inner_number;
        inner_number += 10;
        println!("inner number: {}", inner_number);
    }

    println!("outer number: {}", outer_number);

    let rocket_fuel = 1;
    let mut rocket_fuel_string = String::from("Rocket fuel");
   process_fuel(rocket_fuel,&mut rocket_fuel_string); // This is a copy because we're passing a copy of the value,Datatype here is also an interger an it's strored on the stack, which has an implicity copy 

    println!("Rocket fuel: {}, Rocket fuel String {}", rocket_fuel, rocket_fuel_string);


    let planets = [1,2,3,4,5,6,7,8];// sorry pluto
    let inner_planets_all: &[i32] = &planets[..4];
    println!("inner_planets are {:?}", inner_planets_all);

    let message = String::from("Greetings from Earth");
    println!("message to you is {}",message);

    let last_word = &message[15..15+5];
    println!("Last word is {}", last_word);

    let first_word = get_first_word(&message);
    println!("First word is {}", first_word);
     
     if env::args().len() <= 2{
         println!("Please provide at least two arguments.");
         return;
     }

    for (index,argument ) in env::args().enumerate(){
        println!("Argument {} is: {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("Argument 2 is: {}", arg2);


    let arg1 = env::args().nth(1).unwrap();
    println!("Argument 1 is: {}", arg1);


}


fn get_first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..index];
        }
    }
     // Note 👍study this string slice
     // As a rule fo thumb when writing functons to work with strings ,"without taking ownership, you should use the string slice datatype for the input and output parameters because it has the fexlibilty to also work with string references"
    &s // no spaces found; input is a single word
}

fn process_fuel(mut propellent:i32, roc_string: &mut String)-> usize{
    propellent += 10;
    roc_string.push_str("Rust");
    println!("Propellent Integer: {} Propellent String: {} ", propellent, roc_string);
    roc_string.len()
}

//Restrictions

//When using a mutable refrence, you cannnot create other refrences
//Prevent Data Races

//NOTE : PLs

//You can only create one mutable refrence at a time
//Eg
// let ref1 = &mut var;
//You can create multiple immutable refrences.
//Eg
// let ref1 = & var;
// let ref2 = & var;

//NOTE: Pls ==> 
//You can't create a mutable refrence and any other references.


//SLICE
// Reference to a congtiguous section of a collection