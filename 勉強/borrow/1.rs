fn main(){
    let a = [0,1,2,3];
    let _c = &a;
    let _b = a[0];

    let _d = 1;
    fn aaa(a: u16, _b: u16) -> u16 {
        let d = 2;
        println!("{}",d);
        a
    }
    let e = aaa(3, 4);
    println!("{}",e);
}