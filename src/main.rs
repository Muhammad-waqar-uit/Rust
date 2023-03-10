// use std::collections::{ HashMap};

#[derive(Debug)]
enum MyError{
    Error
}

//err a enum that contains error
//ok value to wrapper containing error
fn divide(divident : i32,divisor:i32)->Result<i32,MyError>{
if divident%divisor !=0{
    Err(MyError::Error)
}else{
    Ok(divident/divisor)
}
}
fn main(){
    //enum results:
    let divide =divide(4,2);
    match divide {
        Ok(v)=>println!("{}",v),
        Err(v)=>println!("{:?}",v)
    }



    // let Divide1: Option<i32>=divide(4, 2);
    // let Divide2: Option<i32>=divide(2, 3);
    // println!("{:?} unwraps to {}",Divide1,Divide1.unwrap());
    // println!("{:?} unwraps to {}",Divide2,Divide2.unwrap())
    //same as  dynamic array
    // let mut vec: Vec<i64>=vec![1,2,3,4,5,6,8];
    // vec.len();
    // vec[0];
    // vec.push(6);
    // vec.remove(1);
    // println!("{:?}",vec);


    //hashmaps maps like keyvalue pair
    // let mut map= HashMap::new();

    // map.insert(0, "Hello");
    // map.insert(1, "World");
    // println!("{:?}",map);

    // match map.get(&0) {
    //     Some(str)=> println!("{}",str),
    //     _=>println!("Doesn't Exist In Map"),
    // }

    // match map.get(&2){
    //     Some(str)=>println!("{}",str),
    //     _=>println!("Doesn't Exist In Map"),
    // }


    // map.remove(&0);
    // println!("{:?}",map);
}