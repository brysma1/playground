fn main() {
    // let x: f64 = 3; <- this doesn't work, it needs the .0 to parse it as a float, otherwise it
    // gets interpreted as a integer
    // let x: f64 = 3f64; <- this does work but I don't think is idiomatic rust
    let x: f64 = 3.0;
    let y: f64 = 5.0;
}
