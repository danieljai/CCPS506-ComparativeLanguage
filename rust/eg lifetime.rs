fn main() {

    let x1 = "asbd";
    
    {
        let x2 = "asdfasdf";
        //let x3 = long(x1,x2);
        println!("{}",long(x1,x2));
        
    }
    
}

fn long<'a> (x:&'a str, y:& str) -> &'a str {
    //if x.len() > y.len() {x} else {y}
    x
}