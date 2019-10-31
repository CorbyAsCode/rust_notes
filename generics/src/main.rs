fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}, p.y = {}", p.x(), p.y());

    let d = Point { x: 3.4, y: 2.3 };
    println!("distance_from_origin: {},{} = {}", d.x(), d.y(), d.distance_from_origin());

    let t1 = Point2 { x: 9, y: 2 };
    let t2 = Point2 { x: "Hi", y: "there" };
    let t3 = t1.mixup(t2);
    println!("t3.x: {}, t3.y: {}", t3.x, t3.y);
}

// Using a generic in a struct
struct Point<T> {
    x: T,
    y: T,
}

// Using a generic in a method
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// Implement a method for f32 types only
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Using many generics
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
