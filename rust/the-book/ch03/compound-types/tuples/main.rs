fn main() {
    // tuples groups various data types and values under a single variable name
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // to access tuples there are two ways, destructuring via patern matching
    let (a, b, c) = tup;

    // or via a dot (.) and an index
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    assert!(a == x);
    assert!(b == y);
    assert!(c == z);

    // tuples without values are called units
    let _unit: ();
    // that shit allows for magic zero size typemasturbation, very crazy
}
