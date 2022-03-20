

fn main() {
    let a = 10;
    // the check_add returns Option comparing to +
    let b = a.checked_add(100).unwrap();
}