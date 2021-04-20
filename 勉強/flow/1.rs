fn main(){
    for i in 0..20 {
        println!("{}",i);
    }

    let mut i = 0;
    loop{
        i = i + 1;
        println!("{}",i);
        if i == 19 {
            break;
        }
    }
    println!("{}",i);

    let mut i = 0;
    while i != 19  {
        i = i + 1;
        //continue;
        println!("{}",i);
    }
    println!("{}",i);

    let i: u32 = 0;
    test(i);
    println!("{}",i);
}

fn test(mut i: u32){
    loop{
        i = i + 1;
        println!("{}",i);
        if i == 20 {
            break;
        }
    }
}