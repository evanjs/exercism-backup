pub fn is_leap_year(year: i32) -> bool {
    let x = year % 4 == 0 && ((year % 400 == 0 && year % 100 == 0) || year % 100 != 0);
    x
}
