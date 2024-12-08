fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    { // create a new scope for z
        let z = y;    // z is a mutable reference to x
        *z += 1; // This is allowed: modifying x through z
        println!("x = {}", x); // Output: x = 6
    }
    *y += 1; // This is also allowed
    println!("x = {}", x); // Output: x = 7
}