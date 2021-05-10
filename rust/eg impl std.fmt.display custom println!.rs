
struct Pt2D {x:f64,y:f64}

trait PointOps {
    fn plus (&self, pt: &Self) -> Self;
    fn print (&self);
}

impl PointOps for Pt2D {
    fn plus(&self, pt:&Pt2D) -> Pt2D {
        Pt2D {
            x: self.x + pt.x,
            y: self.y + pt.y
        }
    }
    
    fn print(&self) {
        println!("2D!!");
        println!("{} {}",self.x, self.y);
    }
}

impl std::fmt::Display for Pt2D {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{},{}>",self.x, self.y)
    }
}

fn main() {

    let p4 = Pt2D {x:1.0, y:1.2};
    let p5 = Pt2D {x:3.0, y:5.2};
    let p6 = p4.plus(&p5);
    
    println!("{}",p6);
    
}