use crate::classes::EntsoeClient;
mod classes;

fn main() -> () {
    let api_key : &str = "<API_KEY>";
    let entsoe_client: EntsoeClient = EntsoeClient { api_key: api_key.into() };

    let start_time: &str = "202207011600";
    let end_time: &str = "202207012300";
    let area: &str = "DE_LU";
    entsoe_client.query_day_ahead_prices(start_time, end_time, area);
}
