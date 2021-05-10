fn main() {
    let inc = |n| n+1;
    let sqr = |n| n+n;
    println!("{}",first_class(inc,5));
    
    // first class on first function
    let mut nums = [1,2,3,4,5,6];
    println!("{:?}", nums);
    array_op(inc, &mut nums);
    println!("{:?}", nums);

    // first class on second function
    array_op(sqr, &mut nums);
    array_op(sqr, &mut nums);
    array_op(sqr, &mut nums);
    println!("{:?}", nums);
    
    
}

// first function
fn first_class<F> (op:F, n:i32) -> i32 where F: Fn(i32)->i32 {
    op(n)
}


// second function
fn array_op<F> (op:F, n:&mut [i32]) -> &mut [i32] where F: Fn(i32)->i32 {
    for i in (0..n.len()) {
        n[i] = op(n[i]);
    }
    n
}