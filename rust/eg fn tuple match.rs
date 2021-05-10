fn main() {
    let c = gc((255,142,12));
    println!("{}",c);
}

fn gc (v: (u8,u8,u8)) -> String {
    match v {
        (255,_,_) => String::from("R"),
        (_,255,_) => String::from("G"),
        (_,_,255) => String::from("B"),
        (_,_,_) => String::from("WHITE"),
    }
}