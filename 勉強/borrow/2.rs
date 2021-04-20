fn main(){
    let _a: i32;
    if true {
        ;
    }else if true {
        ;
    }else {
        ;
    }

    let b = 0;
    match b {
        1 => println!("a"),
        2 => println!("a"),
        3 => println!("a"),
        _ => println!("aaaa")
    }

    let mut c = if true { 2 } else { 3 };
    c = match b {
        1 => 5,
        0 => 7,
        _ => 8,
    };
    print!("{}",c);
}