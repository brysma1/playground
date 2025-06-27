fn main() {
    let fx: f64 = 3.0;
    let fy: f64 = 11.0;
    let ux: u8 = 3;
    let uy: u8 = 11;
    let ix: i8 = 3;
    let iy: i8 = 11;

    println!("--== ADD =--");
    println!("fx + fy = {}", fx + fy);
    println!("ux + uy = {}", ux.wrapping_add(uy));
    println!("ix + iy = {}", ix.wrapping_add(iy));

    println!("--== SUB =--");
    println!("fx - fy = {}", fx - fy);
    println!("ux - uy = {}", ux.wrapping_sub(uy));
    println!("ix - iy = {}", ix.wrapping_sub(iy));

    println!("--== MUL =--");
    println!("fx * fy = {}", fx * fy);
    println!("ux * uy = {}", ux.wrapping_mul(uy));
    println!("ix * iy = {}", ix.wrapping_mul(iy));

    println!("--== DIV =--");
    println!("fx / fy = {}", fx / fy);
    println!("ux / uy = {}", ux / uy);
    println!("ix / iy = {}", ix / iy);

    println!("--== MOD =--");
    println!("fx % fy = {}", fx % fy);
    println!("ux % uy = {}", ux % uy);
    println!("ix % iy = {}", ix % iy);
}
