fn main() {
    // listing 4-5
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");

    // this code passes a string to a function which calculates the lenght of the string
    // returning the string and it's lenght
    // the function returns the string so i can use it after moving the string into the function

    // this approach is very verbose for something that's pretty common, so instead of this i can
    // use a reference denoted by `&`
    // a reference is similar to a pointer in C, in which it contains the memory address of some
    // value, but differs from a pointer due to it garanteeing that it points to a value, while
    // leaving the variable immutable
    // this is called borrowing, since the function 'borrows' the value instead of taking ownership

    let s1 = String::from("alo");

    let len = calculate_length_v2(&s1);

    println!("The lenght of '{s1}' is {len}.");

    // when borrowing a value it can't be modified, just like irl when you borrow something you
    // should return it in the same state as when it was borrowed.
    // in the cases when a borrowed value needs to be mutated, i use a mutable reference via `&mut`

    let mut s1 = String::from("i no like when need to use js"); // now `s1` is mutable so it can be modified

    make_uppercase(&mut s1); // this modifies `s1`

    println!("'{s1}' was lowercase"); // `s1` is still available

    // this is allows me to let functions modify variables without giving up ownership, such thing
    // would lead to easily shooting myself in da feet in C, rust garantees (iThink™) that no foot
    // shooting will happen with references, since if there's already a mutable reference no other
    // reference can take place, or no mutable references can happen if there's a reference that
    // will be used after the mutable borrow happens

    let mut s = String::from("rust is different");
    let s1 = &s;
    let s2 = &s;

    // all ok here

    let mut s3 = &mut s;

    // this is also ok

    // now `s1` and `s2` are not accesible
    // so println!("{_s1} {_s2}"); is a nono

    println!("{s3}");

    // so for references and borrowing there are two rules:
    // - in any moment there can be either one mutable reference, or multiple immutable references
    // - References must always be valid
    // this also means that no reference will be pointing into an invalid address
}

fn make_uppercase(s: &mut String) {
    *s = s.to_uppercase();
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a string

    (s, length)
} // here `s` would be droped and it's memory would be freed if it wasn't returned

fn calculate_length_v2(s: &String) -> usize {
    s.len()
} // here `s` isn't dropped because the value was borrowed
