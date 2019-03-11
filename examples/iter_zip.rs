fn main() {
    let compare_date1 = [2019, 2, 3];
    let compare_date2 = [2019, 3, 2];
    for (cnt, item) in compare_date1.iter().zip(compare_date2.iter()).enumerate() {
        println!("{:?}", cnt);
        println!("{} {}", item.0, item.1);
    }
}
