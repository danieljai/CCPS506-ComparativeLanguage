enum Person {
    Man(String, u8),
    Woman(String,u8)
}

impl Person {
    fn birthyear (&self) {
        
        let a = match *self  {
            Person::Man(_,x) => (2018-x as u16),
            Person::Woman(_,x) => (2018-x as u16)
        };
        println!("how to do things {}",a);
        
    }
}

fn main() {
    let _u1 = Person::Man(String::from("Dee"),23);
    let _u2 = Person::Woman(String::from("Mary"),50);
    _u1.birthyear();
    _u2.birthyear();
}