struct Queue {
    older: Vec<u32>,
    younger: Vec<u32>
}

impl Queue{
    fn new() -> Queue {
        Queue {
            older: Vec::new(),
            younger: Vec::new()
        }
    }

    fn add(&mut self){
        self.older.push(1);
    }

    fn sub(&self){
        println!("jjj");
    }
}

fn main(){
    let i: Vec<u32> = Vec::new();
    //i.push(1);
    println!("{:?}",i);

    let a = 40;
    change(a);
    println!("{}",a);

    let mut q1 = Queue{
        older: Vec::new(),
        younger: Vec::new()
    };

    q1.add();
    q1.younger.push(2);
    q1.sub();

    println!("{:?}",q1.older);
    println!("{:?}",q1.younger);

    let _q2 = Queue::new();
}

fn change(mut _d: u32) {
    _d = 32;
}