use std::fmt::Display;

struct Point<T> {x:T, y:T,}

impl<T> Point<T> {
    fn s_cords(self) -> Point<T> {
        Point {x: self.y, y:self.x}
    }
}

impl<T:Display> Point<T> {
    fn print(&self) {
        println!("{} {}", self.x, self.y);
    }
}

fn main() {
    let pt1 = Point {x:1,y:2};
    pt1.print();
    pt1.s_cords().s_cords().s_cords().s_cords().print();
}