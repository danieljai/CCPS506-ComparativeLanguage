fn main() {

    let color = "Blue";
    
    let print_n = |n|
        for _ in 0..n {
            println!("{}",color);
        };
    
    print_n(5);
    
    println!("{}",color);
    
}