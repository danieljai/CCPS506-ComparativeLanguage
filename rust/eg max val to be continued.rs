fn main() {
    let x = [1,312,21412,125,12,512,512,512,512,5164,62];
    
    println!("{:?}",max_val(&x));
}

fn max_val (arr:&[i32]) -> i32 {
    let mut largest = arr[0];
    for &n in arr.iter() {
        if n > largest {largest = n;}
    }
    largest
}

