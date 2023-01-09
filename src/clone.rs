#[derive(Copy, Clone, Debug)]
struct Point(i32, i32, String);

fn main() {
    let p1 = Point(3, 4, "Cheese");
    let p2 = p1.clone();
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}