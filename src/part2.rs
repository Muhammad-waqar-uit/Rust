fn main(){

    //
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
    //  
    
    // //switch statement
    //     let i=2;
    //     match i {
    //         0=>print!("0"),
    //         1|2=> println!("1,2"),
    //         3..=4=>println!("3,4"),
    //         _=>println!("default")
    //     }
    // let name=String::from("Bird");
    //     let bird=Bird{
    //         name,
    //         attack: 5
    //     };

    //     bird.print_name();
    //     println!("{} {}",bird.can_fly(),bird.is_animal())
}

// struct Bird{
//     name:String,
//     attack:u64
// }

// impl Bird{
//     fn print_name(&self){
//         println!("{}",self.name);
//     }
    
// }
// fn borrowing_slice(arr:[u8;4],slice: &[u8]){
//     println!("{:?}",arr);
//     println!("{:?}",slice);
//     println!("length:{}",slice.len());
//     println!("{} {}",slice[0],slice[1]);
// }

// trait  Animal {
//     fn can_fly(&self)-> bool;
//     fn is_animal(&self)->bool{
//         true
//     } 
// }

// impl Animal for Bird {
//     fn can_fly(&self)-> bool{
//         return true
//     }
//     fn is_animal(&self)->bool{
//         false
//     } 
// }