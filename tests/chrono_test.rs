extern crate chrono;
extern crate failure;

#[test]
fn chrono_compare() -> std::result::Result<(), failure::Error> {
    use chrono::NaiveDate;
    let date_only = NaiveDate::parse_from_str("2019-09-05", "%Y-%m-%d")?;
    let date_only2 = NaiveDate::parse_from_str("2018-09-06", "%Y-%m-%d")?;
    println!("{}", date_only > date_only2);
    Ok(())
}
