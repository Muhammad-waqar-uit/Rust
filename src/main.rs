fn main(){
//     let arr=[0,2,1,3];
// //derefrancing array
//     let slice = &arr[1..3]; //[1,2]

//     borrowing_slice(arr, slice);

    // let str: &str = "Hello World!";
    // let mut string: String = String::from("HelloWorld");
    // let slice = &string[..6];
    // println!("{} {} {}", str, string, slice);

    // string.push('a');
    // let new_slice = &string[..];
    // println!("{} {} {}", str, string, new_slice);

    // let local =string.replace("Hello","Check");
    // println!("{} {} {}", str, local, new_slice);


    // let n=-1;
    // if n>0{
    //     println!("Greater Than zero");
    // }else if n<0{
    //     println!("Less than zero");
    // }else{
    //     println!("Is zero")
    // }

    //for loop in rust
    // for i in 0..6{
    //     println!("{}",i);
    // }
     let mut i=0;
    while i<4{
        println!("{}",i);
        i+=1;
        if i==3 {
            print!("Exit");
            break;
        }
    }
}


// fn borrowing_slice(arr:[u8;4],slice: &[u8]){
//     println!("{:?}",arr);
//     println!("{:?}",slice);
//     println!("length:{}",slice.len());
//     println!("{} {}",slice[0],slice[1]);
// }