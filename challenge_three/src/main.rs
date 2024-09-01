fn info (a: &String) {
    println!("{}", a);
}




fn main() {
   // let a : &str = "?";
    let b : String = "Hello,String".to_string();
    //info(&a);
    info(&b);

}

#[test]
fn str(){
    let input: &str = "Rust";
    info(&input);
}

#[test]
fn string(){
    let input = String::from("Rust");
    info(&input);
}