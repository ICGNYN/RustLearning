fn main(){
    let a: [i32; 5] = [0,1,2,3,4];
    let b: [i32; 5] = a;
    println!("{:?}",a);
    println!("{:?}",b);

    let mut c = Vec::new();
    c.push(1);
    //let d: Vec<i16> = vec![0,0,0];
    let d = vec![0; 10];
    println!("{:?}",d );

    let e = 5.0/3.0;
    println!("{}",e);
}