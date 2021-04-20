struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}

#[derive(Debug)]
struct Broom{
    height: u32,
    health: u32,
    position: (f32, f32, f32),
}

fn main(){
    let width = 1024;
    let height = 576;
    let _image = GrayscaleMap{
        pixels: vec![0; width * height],
        size: (width, height)
    };

    let hokey = Broom {
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0)
    };
    let (hokey1, hokey2) = chop(hokey);
    println!("{:?}",hokey1);
    println!("{:?}",hokey2);
}

fn chop(b: Broom) -> (Broom, Broom) {
    let broom1 = Broom { height: b.height / 2, ..b};
    let c = b;

    let broom2 = Broom { ..broom1 };

    (broom1, broom2)
}