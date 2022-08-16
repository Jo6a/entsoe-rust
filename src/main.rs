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
    let choice_data = DataChoice::ShowAsGraph;

    let resp: Result<Vec<DatetimeValue>, Box<dyn Error>> = match choice_query {
        QueryChoice::QueryDayAheadPrices => {
            ENTSOE_CLIENT.query_day_ahead_prices(START_TIME, END_TIME, AREA)
        }
        QueryChoice::QueryNetPosition => {
            ENTSOE_CLIENT.query_net_position(START_TIME, END_TIME, AREA)
        }
        QueryChoice::QueryLoad => ENTSOE_CLIENT.query_load(START_TIME, END_TIME, AREA),
        QueryChoice::QueryLoadForecast => {
            ENTSOE_CLIENT.query_load_forecast(START_TIME, END_TIME, AREA, None)
        }
        QueryChoice::QueryGenerationForecast => {
            ENTSOE_CLIENT.query_generation_forecast(START_TIME, END_TIME, AREA, None)
        }
        QueryChoice::QueryWindAndSolarForecast => {
            ENTSOE_CLIENT.query_wind_and_solar_forecast(START_TIME, END_TIME, AREA, None)
        }
        QueryChoice::QueryGeneration => ENTSOE_CLIENT.query_generation(START_TIME, END_TIME, AREA),
    };

    match choice_data {
        DataChoice::WriteToCsvFile => {
            HelpFuncs::write_results_csv("results.csv", ";", resp.unwrap());
        }
        DataChoice::WriteToJsonFile => HelpFuncs::write_results_json("results.json", resp.unwrap()),
        DataChoice::ShowAsGraph => HelpFuncs::plot_data(resp.unwrap()),
    };
}
