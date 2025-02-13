fn equal<T>(x: T, y: T) -> bool where T: PartialEq, {
    x == y
}

fn compare(x: &str, y: &str) {
    equal(x, y);
    equal(x.len(), y.len());
}
pub fn main() {
    let (x, y) = ("3","2");
    //compare(&x, &y);
    println!("x = {}, y = {}", x, y);
}
