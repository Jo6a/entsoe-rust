use crate::classes::DatetimeValue;
use crate::classes::EntsoeClient;
use crate::classes::HelpFuncs;
use std::error::Error;
mod classes;

enum QueryChoice {
    QueryDayAheadPrices,
    QueryNetPosition,
    QueryLoad,
    QueryLoadForecast,
    QueryGenerationForecast,
    QueryWindAndSolarForecast,
    QueryGeneration,
}

enum DataChoice {
    WriteToCsvFile,
    WriteToJsonFile,
    ShowAsGraph,
}

fn main() {
    const API_KEY: &str = "<API_KEY>";
    const START_TIME: &str = "202207011600";
    const END_TIME: &str = "202207012300";
    const AREA: &str = "DE_LU";
    const ENTSOE_CLIENT: EntsoeClient = EntsoeClient { api_key: API_KEY };

    let choice_query = QueryChoice::QueryDayAheadPrices;
    let choice_data = DataChoice::WriteToCsvFile;

    let resp: Result<Vec<DatetimeValue>, Box<dyn Error>>;

    match choice_query {
        QueryChoice::QueryDayAheadPrices => {
            resp = ENTSOE_CLIENT.query_day_ahead_prices(START_TIME, END_TIME, AREA);
        }
        QueryChoice::QueryNetPosition => {
            resp = ENTSOE_CLIENT.query_net_position(START_TIME, END_TIME, AREA);
        }
        QueryChoice::QueryLoad => {
            resp = ENTSOE_CLIENT.query_load(START_TIME, END_TIME, AREA);
        }
        QueryChoice::QueryLoadForecast => {
            resp = ENTSOE_CLIENT.query_load_forecast(START_TIME, END_TIME, AREA, None);
        }
        QueryChoice::QueryGenerationForecast => {
            resp = ENTSOE_CLIENT.query_generation_forecast(START_TIME, END_TIME, AREA, None);
        }
        QueryChoice::QueryWindAndSolarForecast => {
            resp = ENTSOE_CLIENT.query_wind_and_solar_forecast(START_TIME, END_TIME, AREA, None);
        }
        QueryChoice::QueryGeneration => {
            resp = ENTSOE_CLIENT.query_generation(START_TIME, END_TIME, AREA);
        }
    };

    match choice_data {
        DataChoice::WriteToCsvFile => {
            HelpFuncs::write_results_csv("results.csv", ";", resp.unwrap())
        }
        DataChoice::WriteToJsonFile => {
            println!("Not implemented yet!")
        }
        DataChoice::ShowAsGraph => {
            println!("Not implemented yet!")
        }
    };
}
