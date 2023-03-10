fn main(){
    let arr=[0,2,1,3];
//derefrancing array
    let slice = &arr[1..3]; //[1,2]

    borrowing_slice(arr, slice);
}

fn borrowing_slice(arr:[u8;4],slice: &[u8]){
    println!("{:?}",arr);
    println!("{:?}",slice);
    println!("length:{}",slice.len());
    println!("{} {}",slice[0],slice[1]);
}