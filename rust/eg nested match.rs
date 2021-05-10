fn main() {
    let c = gc((255,142,12));
    println!("{}",c);
}

fn gc (v: (u8,u8,u8)) -> String {
    match v {
        (x,_,_) => {
            if x > 200 {String::from("bright") }
            else {String::from("kinda dark") }
        }
    }
}