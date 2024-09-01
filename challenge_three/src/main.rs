fn info <T: std::fmt::Display>(a: &T) {
    println!("{}", a);
}

// fn info(a: &str) {
//     println!("{}", a);
// }




fn main() {
   let a : &str = "?";
    //let b : &String = &"Hello,String".to_string();
    //let input : u32 = 200;
    //let vectord: String = "Hello";
    //info(&a);
    info(&a);
    //info2(&vectord);

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


#[test]
fn u32dd(){
    let input : u32 = 200;
    info(&input);
}


// #[test]
// fn chars() {
//     let input = 'r';
//     info(&input);
// }

// #[test]
// fn cstring() {
//     use std::ffi::{CString};
//     let input = CString::new("Rust").unwrap();
//     info(&input);
// }

// #[test]
// fn path() {
//     use std::path::Path;
//     let input = Path::new("/tmp/rust");
//     info(input);
// }