enum Person<T, E> {
    name(T),
    sex(E)
    
}
fn main() {
    let mut _u:Person<&str,String> = Person::name("Andy");
    _u = Person::sex(String::from("male"));
    
}

