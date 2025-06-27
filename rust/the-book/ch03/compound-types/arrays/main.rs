fn main() {
    // arrays are arrays, nothing crazy, fixed size and stored in the stack
    // rust does have Vectors for dynamic sizes and heap stored values, so i need to keep that in
    // mind cuz i'm not used to use that
    let mut a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = [1, 2, 3, 4, 5]; // <- explicit type and size anotation

    // arrays accesses are via square brackets ([]) and a 0 based index
    a[0] = 6;
    assert!(a[0] != b[1]);

    // out of bounds access to arrays throw errors on compilation and panic in runtime
    // b[10] = 69; <- this throws error
}
