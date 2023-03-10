fn main() {
    // let a = 10;
    // let b= 5;
    // println!("Hello, world!,{} {}",a,b);

    //unsinged integer
    //u8,u16,u32,u64,u128
    // let unsigned: u8=100;

    // //signed interger
    // //i8,i16,i32,i64,i128
    // let signed:i8=-10;

    // let float: f32=1.2;

    // println!("unsigned: {} signed: {}, float: {}",unsigned,signed,float);

    // //char con only be
    // let letter="CHARACTER01";
    // let emoji="\u{1F600}";
    // println!("Letter : {}, Emoji: {}",letter,emoji);

    // let is_true:bool= true;
    // println!("IsTrue: {}",is_true);

    let arr:[u8;3]=[1,2,3];
    let other_arr:[u8;5]=[100,2,2,2,2];

    println!("Index: {},length: {}",arr[0],other_arr.len());

//print structure of the array and other objects
    println!("{:?}",other_arr);
}