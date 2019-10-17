
use std::io;
pub fn sum(){
        let mut a = String::new();
        println!("Enter number of i32 type");
        io::stdin().read_line(&mut a);
        let mut b = String::new();
        println!("Enter number of i32 type");
        io::stdin().read_line(&mut b);
        let _type1 : i32 =a.trim().parse().unwrap();
        let _type2 : i32 =b.trim().parse().unwrap();
        let total : i32 =_type1 as i32  +_type2 as i32 ;
        println!("{}", total)
           
}
pub fn difference(){
        let mut a = String::new();
        println!("Enter number of i32 type");
        io::stdin().read_line(&mut a);
        let mut b = String::new();
        println!("Enter number of i32 type");
        io::stdin().read_line(&mut b);
        let _type1 : i32 =a.trim().parse().unwrap();
        let _type2 : i32 =b.trim().parse().unwrap();
        let total : i32 =_type1 as i32  - _type2 as i32 ;
        println!("{}", total)
           
}
pub fn devision(){
        let mut a = String::new();
        println!("Enter number of i32 type");
        io::stdin().read_line(&mut a);
        let mut b = String::new();
        println!("Enter number of i32 type");
        io::stdin().read_line(&mut b);
        let _type1 : f32 =a.trim().parse().unwrap();
        let _type2 : f32 =b.trim().parse().unwrap();
        let total =(_type1 /_type2 ) as f32  ;
        println!("{}", total)
           
}
pub fn multiplication(){
        let mut a = String::new();
        println!("Enter number of i32 type");
        io::stdin().read_line(&mut a);
        let mut b = String::new();
        println!("Enter number of i32 type");
        io::stdin().read_line(&mut b);
        let _type1 : i32 =a.trim().parse().unwrap();
        let _type2 : i32 =b.trim().parse().unwrap();
        let total =_type1 *_type2   ;
        println!("{}", total)
           
}
