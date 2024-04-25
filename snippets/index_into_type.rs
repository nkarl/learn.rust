fn main() {
    let b1: Vec<i32> = vec![1, 2, 10];
    let b2: Vec<i32> = vec![2, 3, 20];
    let bookings: Vec<&Vec<i32>> = vec![&b1, &b2];

    for (from, to, seats) in bookings.iter().map(|b| (&b[0], &b[1], &b[2])) {
        println!("{from} {to} {seats}");
    }
}
