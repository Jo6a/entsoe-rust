use crate::classes::EntsoeClient;
mod classes;

fn main() {
    const API_KEY: &str = "<API_KEY>";
    const START_TIME: &str = "202207011600";
    const END_TIME: &str = "202207012300";
    const AREA: &str = "DE_LU";
    const ENTSOE_CLIENT: EntsoeClient = EntsoeClient { api_key: API_KEY };

    let _resp = ENTSOE_CLIENT.query_day_ahead_prices(START_TIME, END_TIME, AREA);
    let _resp = ENTSOE_CLIENT.query_net_position(START_TIME, END_TIME, AREA);
    let _resp = ENTSOE_CLIENT.query_load(START_TIME, END_TIME, AREA);
}
