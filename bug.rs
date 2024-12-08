fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y;    // z is an immutable reference to y
    *y += 1;     // This is allowed: modifying x through y
    println!("x = {}", x); // Output: x = 6
    *z += 1;    // ERROR: cannot assign to immutable reference
}