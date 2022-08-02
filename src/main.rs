use crate::classes::DatetimeValue;
use crate::classes::EntsoeClient;
use chrono::NaiveDateTime;
use std::error::Error;
mod classes;

fn main() -> Result<(), Box<dyn Error>> {
    const API_KEY: &str = "<API_KEY>";
    const ENTSOE_CLIENT: EntsoeClient = EntsoeClient { api_key: API_KEY };

    const START_TIME: &str = "202207011600";
    const END_TIME: &str = "202207012300";
    const AREA: &str = "DE_LU";
    let resp = ENTSOE_CLIENT.query_day_ahead_prices(START_TIME, END_TIME, AREA);

    let start_time_dt: NaiveDateTime =
        NaiveDateTime::parse_from_str(START_TIME, "%Y%m%d%H%M").unwrap();
    let mut response_vector: Vec<DatetimeValue> = vec![];
    response_vector.push(DatetimeValue {
        dt: start_time_dt,
        val: 2.34,
    });
    println!("{}", response_vector[0].val);
    Ok(())
}
