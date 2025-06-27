// main function is the entrypoint of the program, as usual
fn main() {
    let x = 609;
    let y = 41920;
    println!("{}", func_urself(x, y));
}

// function names in snake case plz
// return type given by -> (type)
// last expression result is returned implicitly if there's no semicolo at the end, so no return
// keyword needed there
// return is mainly used when returning in other points of the function body
fn func_urself(who: i32, asked: i32) -> i32 {
    who.saturating_mul(2).saturating_mul(asked)
}
