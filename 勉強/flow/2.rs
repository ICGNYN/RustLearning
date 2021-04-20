fn main(){
    let a = vec![1,2,3,4,5,6,7];
    for i in &a[2..] {
        println!("{}",i);
    }

    let x = 17;
    let _y = x as usize;

    let is_even = |b: u32| b % 2 == 0;
    println!("{}",is_even(4));

    let mut e = 50;
    check(&mut e);
    println!("{}",e);
}

fn check(d: &mut usize) {
    *d = 100;
    println!("{}",d);
}