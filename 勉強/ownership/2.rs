fn main(){
    let mut a = vec![0,0,0];
    a.push(1);
    let c = a[0];
    let mut s = vec![];
    for i in a{
        s.push(i);
    }
    a = vec![2,3,4];
    
    println!("{:?}",a);
    println!("{:?}",a);
    //println!("{:?}",b);
    println!("{}",c );
    println!("{:?}",s);
}