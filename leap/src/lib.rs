pub fn is_leap_year(year: u64) -> bool {
    match (year % 400, year % 100, year % 4) {
        (0, _, _) => true, // Is divisible by 400
        (_, 0, _) => false, // Is divisible by 100 and 400
        (_, _, 0) => true, // Is divisible by 4, 100 and 400
        _ => false
    }
}
