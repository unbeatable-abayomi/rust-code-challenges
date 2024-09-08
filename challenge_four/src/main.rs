//Case Sensetive Sort.

fn sort_usernames<T: AsRef<str> + Ord>(usernames: &mut Vec<T>){
   usernames
   .sort_by_cached_key(|x: &T| x.as_ref().to_lowercase());
}


fn main() {
   let mut users : Vec<&str> = vec!["Todd","Amy","mike99","Jennifer","alison"];
   println!("unsorted: {:?}", &users);
   sort_usernames(&mut users);
   println!("sorted: {:?}", &users );
}

#[test]
fn five_users(){
    let mut users : Vec<&str> = vec!["Todd","Amy","mike99","Jennifer","alison"];
    let sorted : Vec<&str> = vec!["alison","Amy","Jennifer","mike99","Todd"];
    sort_usernames(&mut users);
    assert_eq!(users, sorted);
}
