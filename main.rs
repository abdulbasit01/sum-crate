fn main(){
    println!("Check" );
    sum()
    
}
use std::io;
fn sum (){
    println!("How many numbers you want to add");
    let mut a = String::new();
    io::stdin().read_line(&mut a);
    let number : i32 =a.trim().parse().unwrap();
    let mut i =0;
    while i !=number{
        let mut a = String::new();
        io::stdin().read_line(&mut a);
        let mut _type : i32 =a.trim().parse().unwrap();
        let mut number=_type;
        number+=_type;
        _type=number;
        println!("{}",_type ) ;
        // println!("{}",_type );
        i=i+1
    }
    
}
// fn difference (){
//     println!("How many numbers you want to add");
//     let mut a = String::new();
//     io::stdin().read_line(&mut a);
//     let number : i32 =a.trim().parse().unwrap();
//     let mut i =0;
//     while i !=number{
//         let mut a = String::new();
//         io::stdin().read_line(&mut a);
//         let mut _type : i32 =a.trim().parse().unwrap();
//         let mut number=_type;
//         number-=_type;
//         _type=number;
//         println!("{}",_type ) ;
//         // println!("{}",_type );
//         i=i+1
//     }
    
// }