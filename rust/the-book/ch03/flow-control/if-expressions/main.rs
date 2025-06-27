fn main() {
    // if statements are a statement of an if and a contidion that can be true or false and if the
    // condition is true it does something and if not it does something else, and if then
    // iffiifififiiifififiifififififififiif
    if true {
        println!("condition true");
    } else {
        println!("condition false");
    }

    // conditions need to compute to true or false, no leaving numbers there
    // let x = 3;
    // if x { <- this doesn't work
    //   // do something... idk
    // }

    // there's also else if statements, so more of the same stuff, nothing crazy

    // asignation from an if statement is also possible
    let condition = true;
    let number = if condition { 5 } else { 6 }; // <- cool shit iguess

    println!("The value of number is: {number}");
}
