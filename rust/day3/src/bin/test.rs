fn main() {
    let full = vec!['e', 'a', 'd', 'o'];
    let slice = ['d', 'o', '(', ')'];
    let empty_slice: &[char] = &[];
    let start_index: usize = 2;
    let extract = match full.get(start_index..start_index + 4) {
        Some(val) => val,
        None => empty_slice,
    };
    // assert_eq!();
    println!("ok {}", extract == slice)
}
