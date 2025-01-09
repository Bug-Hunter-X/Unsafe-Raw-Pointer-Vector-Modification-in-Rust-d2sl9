fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; // Safe and preferred way to modify the first element
    println!( "{:?}", v);

    // Another safe method using get_mut()
    if let Some(val) = v.get_mut(1) {
        *val = 5;
    }
    println!( "{:?}", v);
}