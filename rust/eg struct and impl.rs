struct Person {
    name: String, age: u16
}

impl Person {
    fn birthyear(&self) -> u16 {
        2018 - self.age 
    }
    
    fn oneyear(&self) -> u16 {
        self.age - 1
    }
}

fn main() {
    let _u1 = Person{name:String::from("Tim"),age:37};
    
    println!("my birth year is {} ",_u1.birthyear());
    println!("one year {} ",_u1.oneyear());
}

