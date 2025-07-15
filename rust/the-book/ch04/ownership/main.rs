use std::string;

fn main() {
    // scopes are defined by "blocks," which are delimited by curly braces, so,  the content's of the
    // 'main' function are contained within the current block

    // a variable only comes into scope after it has been defined, so:
    // from this line backwards, 's' is not valid
    let s = "alooOO"; // this expression is the responsible for making 's' valid in the current scope
                      // from this line onwards 's' is valid

    // the '{' found under marks the start of a new block
    {
        // from this line backwards, inside the current block, 'x' is not valid
        let x = 5; // this makes 'x' valid in the current block
                   // from this line onwards, inside the current block, 'x' is valid

        // here 's' is still valid, since the current block is inside the block where 's' was defined,
        // in other words, the current scope is still under the scope where 's' was made valid
    }
    // the '}' found above marks the end of the block
    // that means the '}' ends the scope where 'x' was valid, when that block ended the scope ends with it
    // making 'x' invalid outside that scope, in other words (again), the scope of 'x' WAS under
    // the current scope, so 'x' is not able to go into a higher scope

    // up until now the simple datatypes we have used are stored in the stack, this because they
    // have a known size, which allows us to know exactly how much memory we need, but not all data
    // has a defined size, in those data is stored in the heap.

    let s = String::from("aloO"); //  the String type stores data both in the stack AND the heap
                                  // the heap stores the actual string
                                  // while the stack stores the pointer to the string, it's lenght, and the capacity

    {
        let a = String::from("guess what...");
        // this would store a new string in the heap and the corresponding values in the stack
        // but when this scope ends, drop is called implicitly, since the variable won't be
        // available outside this scope, freeing the memory used by the actual string in the heap
        // drop(a); <- this is implicit and doesn't need to be written in code
    }

    // now let's set the following
    let a = 10;
    let b = a;

    // in some programing languages 'b' would simply be a reference to 'a', in others (like rust)
    // the value of 'a' is copied into 'b', this is due to the ownership rules where every value must
    // have a single owner, so 'a' owns the first 10, and for 'b' the 10 is copied and 'b' owns the copy

    // this works for the scalar values i've worked with up until now but for more other types (like String)
    // this won't always work.

    let s1 = String::from("js sucks"); // this allocates memory both in the heap and the stack
    let s2 = s1; // this would be the data in the stack

    // since the data in the stack contains a pointer to the heap (where the actual string is
    // stored) the value would have 2 owners, which breaks the rules of ownership
    // so the stack data (pointer, size and capacity) is not copied but only moved from 's1' to 's2'
    // invalidating 's1' and leaving only 's2' valid

    // now, what if the values from a String change?
    let mut s = String::from("and"); // this stores the pointer to "and" in the heap, the lenght (3) and the capacity (3), all in the stack
    s = String::from("it sucks a lot"); // this would change the three values,

    // what happened to the pointer to "and" in the heap? there's no pointer to it, meaning it
    // would be floating in the memory (memory leak), or that would be the case in other languages (like C)
    // rust detects this and implicitly calls drop on the "and" string and its memory will be freed

    // then, what if i do want to have both String vars availiable?
    // for that i would have to clone the String from one variable to the other

    let s1 = String::from("i liek piza"); // this creates the String
    let s2 = s1.clone(); // here the data in the heap is replicated in another address, the lenght and capacity are copied but the pointer is different

    // this (in theory and in my mind with my current understanding) would be the same as the following

    let s1 = String::from("bnunies are cute");
    let s2 = String::from("bnunies are cute");

    // okay, but then why in the case of a simple scalar value the value is copied by default and
    // both variables are valid? like in the following case

    let x = 9;
    let y = x;
    // here both x AND y are valid

    // this is because, like before, the size is known at compile time, the values are stored in the
    // stack, and working with memory is faster done in the stack. this is why simple scalar values
    // implement the 'Copy" trait, which allows for implicit cloning of variables, so it is good to keep that in mind

    // Currently i'm inside the main function, but what about when working with other functions??
    // well, it's kinda simple

    // when a variable is passed to a function, it is either copied or moved into the function
    // variable, depending if it implements the Copy trait, just like the previous examples

    // so, the following
    let s1 = String::from("i liek cats");
    takes_ownership(s1); // this function takes ownership of 's1'
                         // the "takes_ownership" function borrows it's value inside its scope, so when that scope ends the
                         // memory is freed.

    // then if we want to pass a variable to another function and use the value after that i need to
    // return the value, essentialy borrowing back the value
    // // my guess is that this is not idiomatic
    let mut s1 = String::from("i liek birbs");
    s1 = takes_and_returns_ownership(s1);

    // and in the case of the simple scalar types, the value is copied
    let x = 6;
    makes_copy(x);
    // in this case 'x' is still availiable because of the copy trait
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn takes_and_returns_ownership(some_string: String) -> String {
    println!("{some_string}");
    some_string
}

fn makes_copy(some_int: u32) {
    println!("{some_int}");
}
