fn main(){
//     let arr=[0,2,1,3];
// //derefrancing array
//     let slice = &arr[1..3]; //[1,2]

//     borrowing_slice(arr, slice);

    let str: &str = "Hello World!";
    let mut string: String = String::from("HelloWorld");
    let slice = &string[..6];
    println!("{} {} {}", str, string, slice);

    string.push('a');
    let new_slice = &string[..];
    println!("{} {} {}", str, string, new_slice);

    let local =string.replace("Hello","Check");
    println!("{} {} {}", str, local, new_slice);
}


// fn borrowing_slice(arr:[u8;4],slice: &[u8]){
//     println!("{:?}",arr);
//     println!("{:?}",slice);
//     println!("length:{}",slice.len());
//     println!("{} {}",slice[0],slice[1]);
// }