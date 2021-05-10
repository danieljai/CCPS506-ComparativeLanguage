fn main() {
    let a = String::from("hi,");
    let b = String::from("there");

    // Q1.
    println!("{}",lab4_1( &a, &b));

    // Q2.
    println!("{}", lab4_2(&[1.1,1.2,1.3,2.6,2.8,3.4,10.8,1.1,4.2],(1,6)) );

    // Q3.
    let mut arr1 = [1,-2,3,-4,5,-6,6,0,0,-112];

    // printing array before
    for i in 0..arr1.len() {
        print!("{} ",arr1[i] );
    }
    println!("");


    lab4_3(&mut arr1);

    // printing array after
    for i in 0..arr1.len() {
        print!("{} ",arr1[i] );
    }
    println!("");

}

fn lab4_1(a: &str, b: &str) -> String {
    let mut c = String::from(a);
    c.push_str(b);
    c
}

fn lab4_2(a: &[f32], bounds: (usize, usize)) -> f32 {
    let mut result = 0.0;
    for i in bounds.0..bounds.1 {
        result = result + &a[i];
    }
    result
}

fn lab4_3(arr:&mut [i32]) { 
    for i in 0..arr.len() {
        
        arr[i] = if arr[i] == 0 {0} 
                    else { arr[i]/arr[i].abs()};    
    }
}