use crate::classes::EntsoeClient;
use crate::classes::DatetimeValue;
use chrono::{DateTime, NaiveDateTime, Utc, FixedOffset};
use std::error::Error;
mod classes;

fn main() -> Result<(), Box<dyn Error>> {
    let api_key : &str = "<API_KEY>";
    let entsoe_client: EntsoeClient = EntsoeClient { api_key: api_key.into() };

    let start_time: &str = "202207011600";
    let end_time: &str = "202207012300";
    let area: &str = "DE_LU";
    let resp = entsoe_client.query_day_ahead_prices(start_time, end_time, area);
    
    let start_time_dt: NaiveDateTime = NaiveDateTime::parse_from_str(
        start_time, "%Y%m%d%H%M").unwrap();
    let mut response_vector: Vec<DatetimeValue> = vec![];
    response_vector.push(DatetimeValue { dt: start_time_dt, val: 2.34 });
    println!("{}", response_vector[0].val);
    Ok(())
}
