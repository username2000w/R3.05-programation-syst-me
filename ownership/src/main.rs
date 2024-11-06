#[derive(Clone)]
#[derive(Copy)]
struct Rectangle {
    length: f64,
    width: f64,
}

fn perimeter(rect : Rectangle) -> f64 {
    2.0 * (rect.length + rect.width)
}

fn perimeter2(rect : &Rectangle) -> f64 {
    2.0 * (rect.length + rect.width)
}

fn average(a : f64, b : f64) -> f64 {
    (a+b) / 2.0
}

fn print_references() {
    let x = 18;
    // let ref1 = &x;
    // let ref2 = &x;
    let mut ref3 = &x;
    let mut ref4 = &x;
    println!("{} {}", ref3, ref4)
}

fn swap(x: &mut f64, y: &mut f64) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

fn main() {
    let x = 4.2;
    let y = 5.8;

    println!("{}", average(x, y));

    let rect = Rectangle{length: 2.0, width: 6.0};
    
    println!("{}", perimeter(rect.clone()));

    let rect2 = Rectangle{length: 2.0, width: 6.0};
    
    println!("{}", perimeter2(&rect2.clone()));

    println!("---------------------------------------------------");

    print_references();

    let mut x = 4.0;
    let mut y = 5.0;
    swap(&mut x, &mut y);

    println!("x: {}, y: {}", x, y)
}