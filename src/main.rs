use umbralux::core::{Point, Vector};

fn main() {

    let pt = Point::new(1.0, 2.0, 3.0);
    let v = Vector::new(-1.0, -2.0, -3.0);

    let pt2 = pt + v;

    println!("{:?}", pt2);

}