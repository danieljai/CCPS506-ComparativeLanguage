struct Pt2D {x:f64,y:f64}
struct Pt3D {x:f64,y:f64,z:f64}

trait PointOps {
    fn plus (&self, pt: &Self) -> Self;
    fn print (&self);
}

impl PointOps for Pt3D {
    fn plus(&self, pt:&Pt3D) -> Pt3D {
        Pt3D {
            x: self.x + pt.x,
            y: self.y + pt.y,
            z: self.z + pt.z
        }
    }
    
    fn print(&self) {
        println!("I like to do whatever i want");
        println!("{} {} {}",self.x, self.y, self.z);
    }
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

fn main() {
    let p1 = Pt3D {x:1.0, y:1.2, z:3.3};
    let p2 = Pt3D {x:3.0, y:5.2, z:8.0};
    let p3 = p1.plus(&p2);
    p3.print();
    
    let p4 = Pt2D {x:1.0, y:1.2};
    let p5 = Pt2D {x:3.0, y:5.2};
    let p6 = p4.plus(&p5);
    p6.print();
}