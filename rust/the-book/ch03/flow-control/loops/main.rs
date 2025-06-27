fn main() {
    // loop is just a while true, it runs until process is stopped, user input, or it finds a break
    // statement
    let mut counter: i8 = 0;

    // loops can be used to do some operations and get some result form it's calculations
    let result = loop {
        println!("js sucks a lot");
        if counter > 9 {
            break counter * 2; // <- here this returns the double of counter and asigns it to result
        }
        counter += 1;
    };

    println!("{}", result);

    let mut counter_a = 0;
    let mut counter_b = 0;

    // loops can be named to break from inner loops inside the loop, so it doesn't keep looping
    // this avoid having a flag bonanza to just break out of nested loops
    'loop1: loop {
        println!("what sucks?");
        loop {
            println!("JS!!!!");
            if counter_a >= 8 {
                break 'loop1; // <- this breaks out of the whole loop and allows the program to continue execution
            }
            if counter_b >= 3 {
                counter_a += 1;
                counter_b = 0;
                break;
            }
            counter_b += 1;
        }
    }

    // there are also the classic while loops
    counter = 0;
    while counter < 5 {
        print!("{}... ", counter);
        counter += 1;
    }
    println!("");

    // and for-in loops
    let a = [1, 2, 3, 4, 5];
    for e in a {
        println!("{}", e);
    }

    // also note we can denote ranges via x..y
    for e in (1..5).rev() {
        println!("{}", e);
    }
}
