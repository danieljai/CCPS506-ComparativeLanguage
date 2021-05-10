fn main() {

    let mut num = 1;
    {    
        let mut inc = || num += 1;
        
        inc().inc();
    }
    //num += 1;
    
    println!("{}", num)
}