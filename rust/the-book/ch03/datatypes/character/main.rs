fn main() {
    // char cariables are single characters inside single quotes, similar to C, but not limited to
    // ASCII characters, instead encodes UTF-8, which can vary from 1 to 4 bytes (as far as i
    // understand now), but here in rust all chars have a 4 byte width, aligned to 4 bytes in
    // memory
    let _c = 'z'; // implicit type inference
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';
}
