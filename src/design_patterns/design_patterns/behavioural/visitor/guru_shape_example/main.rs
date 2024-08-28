
trait Shape {
    fn moving(&self, x: i32, y: i32);
    fn draw(&self);
    //fn accept(v: &impl Visitor);
    fn accept<V: Visitor>(&self, v: &V);
}

struct Dot {

}

impl Shape for Dot {
    fn moving(&self, x: i32, y: i32) {
        println!("dot moving x: {}, y: {}:", x, y);
    }

    fn draw(&self) {
        println!("dot draw");
    }

    fn accept<V: Visitor>(&self, v: &V) {
        println!("dot accept()");
    }
}

struct Circle {

}

impl Shape for Circle {
    fn moving(&self, x: i32, y: i32) {
        println!("circle moving: {}, {}", x, y);
    }

    fn draw(&self) {
        println!("circle draw");
    }

    fn accept<V: Visitor>(&self, v: &V) {
        v.visit_circle(self);
        println!("circle accept");
    }
}

trait Visitor {
    fn visit_dot(e: Dot);
    fn visit_circle(c: Circle);
}

struct XMLExportVisitor {

}

impl Visitor for XMLExportVisitor {
    fn visit_dot(e: Dot) {
        e.moving(5, 2);
        e.draw();

    }

    fn visit_circle(c: Circle) {
        c.moving(1, 1 );
        c.draw();
    }
}



fn main() {
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    shapes.push(Box::new(Circle {}));
    shapes.push(Box::new(Dot {}));

    let xml_export_visitor = XMLExportVisitor {};

    for e in shapes.iter() {
        e.draw();
    }

}