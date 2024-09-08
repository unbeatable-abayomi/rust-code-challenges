


fn main() {

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