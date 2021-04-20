fn main(){
    let mut a = 5;
    let b = &mut a;
    *b = 1;
    //println!("{}",&a );
    println!("{}",b );
    println!("{}",b );

    let c = vec!["aaa","bbb","ccc"];
    let d = c[0];
    let e = &c;
    println!("{:?}",c);
    println!("{}", d)
}