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
    
    fn printname (&self) {
        let a = match &self {
            Person::Man(x,_) => x.to_string(),
            Person::Woman(x,_) => x.to_string()
        };
        
        
        println!("My name is {}", a);
    }
}

fn main() {
    let _u1 = Person::Man(String::from("Dee"),23);
    let _u2 = Person::Woman(String::from("Mary"),50);
    //_u1.birthyear();
    //_u2.birthyear();
    
    //_u1.printname();
    //println!("{}",_u1);
    outside(&_u1);
    outside(&_u2)
}


fn outside (v: & Person) {
        let a = match &v {
            Person::Man(x,_) => x.to_string(),
            Person::Woman(x,_) => x.to_string()
        };
        
        
        println!("My name is {}", a);
}