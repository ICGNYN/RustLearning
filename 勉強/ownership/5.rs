fn main(){
    let a = vec![0,0,0];
    let c = vec![0,0,0];
    let mut b = &a;
    println!("{:?}",b);
    b = &c;

    //a.push(1);

    for _i in &a{
        let mut _e = 5;
    }

    if a == c {
        let _d = 4;
    }

    //println!("{:?}",a);
    println!("{:?}",a);
    println!("{:?}",b);

}