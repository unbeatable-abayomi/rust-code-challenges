
fn median(a: Vec<f32>) -> Option<f32>{
    // - empty
    // - odd number of elements in the list
    // - even number of elements in the list

if a.is_empty() {
    return None;
}

// TODO: sort

let n_elements: usize = a.len();
let middle: usize = n_elements /2;

let med: f32 = if n_elements % 2 == 0{
    //even
    (a[middle - 1] + a[middle]) / 2.0
}else{
    //odd
    a[middle] 
};

Some(med)
    // let len = a.len();
    // if len == 0 {
    //     return None;
    // }



    // let mut sorted_list = a.clone();
    // sorted_list.sort_unstable();

    // match len % 2 {
    //     0 => {
    //         let mid1 = len / 2;
    //         let mid2 = mid1 - 1;
    //         Some((sorted_list[mid1] + sorted_list[mid2]) / 2.0)
    //     }
    //     1 => Some(sorted_list[len / 2]),
    //     _ => None,
    // }
}
fn main() {


    let answer: Option<f32>  = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}",answer);
}

#[test]

fn empty_list(){
    let input: Vec<f32> = vec![];

    let expected_output:Option<f32> = None;
    let actual_output:Option<f32> = median(input);

    assert_eq!(expected_output, actual_output);
}


#[test]
fn even_lenght(){
    let input: Vec<f32> = vec![1.0, 2.0, 3.0];

    let expected_output:Option<f32> = Some(2.0);
    let actual_output:Option<f32> = median(input);

    assert_eq!(expected_output, actual_output);
}

#[test]

fn sorted_list(){
    let input: Vec<f32> = vec![3.0, 2.0,2.0, 2.0];

    let expected_output:Option<f32> = Some(2.0);
    let actual_output:Option<f32> = median(input);

    assert_eq!(expected_output, actual_output);
}

//unsorted_list