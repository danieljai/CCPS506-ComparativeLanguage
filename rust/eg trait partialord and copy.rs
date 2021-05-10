fn main() {
    let a = [1,23,1,2412,412,4325,2345,1245,124];
    let b = ["afeaf","asdf","efw3"];
    
    println!("{}",max_val(&a));
    println!("{}",max_val(&b));
    
}

fn max_val<T: PartialOrd + Copy> (arr: &[T]) -> T {
    let mut largest = arr[0];
    for &n in arr.iter() {
        if n > largest  {largest = n;}
    }
    largest
}
