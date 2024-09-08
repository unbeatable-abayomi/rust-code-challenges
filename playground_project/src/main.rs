


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
}