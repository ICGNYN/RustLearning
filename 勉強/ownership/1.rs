fn print_padovan(){
    let mut padovan = vec![1,1,1];  //allocated
    for i in 3..10{
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("p(1..10) = {:?}",padovan);
}   //padovan is doropped here

fn main(){
    print_padovan();
}