use std::env;
use std::fs;
#[derive(Debug)]
struct Shuttle{
    name: String,
    crew_size: u8,
    propellent:f64
}


impl Shuttle{
    fn get_name(&self) -> &str{
        &self.name
    }

    fn add_fuel(&mut self,gallons: f64){
         self.propellent += gallons;
    }

    fn new(name: &str) -> Shuttle{
        Shuttle{
            name: name.to_string(),
            crew_size: 7,
            propellent: 0.0
        }
    }
}

//Tuple structs
struct Color(u8,u8,u8); //RGB
struct Point(u8,u8,u8); //XYZ


fn get_y(p: Point) -> u8{
    p.1
}
fn main() {

let red = Color(255,0,0);
println!("First value {}", red.0);
println!("Second value {}", red.2);

//let y = get_y(red); Would fail cos expecting Point not color
let coord = Point(4,5,6);
let y = get_y(coord);
println!("y is {}", y);
    // A constructor function to create new shuttle
    let a_better_shuttle = Shuttle::new("LetRide");

    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 17,
        propellent: 835858.0
    };

    let vechilce_name = vehicle.get_name();
    println!("name is {}", vehicle.name);
    println!("name is {}", vechilce_name);
    vehicle.name = String::from("Altatins");
    println!("Whole vehicle {:?}", vehicle);
    println!("propellent {:?}", vehicle.propellent);
    vehicle.add_fuel(10000.0);
    println!("propellent {:?}", vehicle.propellent);
    vehicle.crew_size = 200;
    let vechile2 = Shuttle{
        name: String::from("Atlas"),
        ..vehicle
    };

    println!("Whole vehicle seconf2 {:?}", vechile2);
    println!("Whole vehicle {:?}", vehicle);
    if env::args().len() < 2 {
        eprintln!("Program requires two arguments: <file path> <search name>");
        std::process::exit(1);
    }
    let file_path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();

    for line in fs::read_to_string(file_path).unwrap().lines() {
        if line == search_name {
            println!("{} did walk on the Moon!", search_name);
            return;
        }
    }

    println!("{} did NOT walk on the Moon... YET!", search_name);
}