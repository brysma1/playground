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
}
