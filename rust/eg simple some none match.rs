fn main() {
    let v1 = Some(5);
    
    let veval = match v1 {
        Some(x) => x,
        None => 0
    };
    
    println!("{}",veval*veval);
}
