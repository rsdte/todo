trait Shape {
    fn draw(&self);
}

enum ShapeType{
    Rectange,
    Circle,
}

struct Rectange;
struct Circle;

impl Shape for Rectange {
    fn draw(&self) {
        
    }
}

impl Shape for Circle {
    fn draw(&self) {
        
    }
}

struct ShapeFactory;
impl ShapeFactory {
    fn new_shape(s: &ShapeType) -> Box<dyn Shape> {
        match s {
            ShapeType::Circle => Box::new(Circle),
            _ => Box::new(Rectange)
        }
    }
}

fn main() {
    let shape = ShapeFactory::new_shape(&ShapeType::Circle);
    shape.draw();
}