use std::vec;
use std::{collections::HashMap};
use std::error::Error;
use chrono::{NaiveDateTime};
use crate::classes::Mappings;

#[derive(Debug)]
pub struct EntsoeClient {
    pub api_key: String
}

pub struct DatetimeValue {
    pub dt: NaiveDateTime,
    pub val: f32
} 

impl EntsoeClient {
    pub fn basic_request(&self, start_time: &str, end_time: &str, params: HashMap<&str, &str> ) -> Result<String, Box<dyn Error>> {
        // TODO build request with the parameters, not hard-coded
        let req : String = format!("https://transparency.entsoe.eu/api?documentType=A44&in_Domain=10Y1001A1001A82H&out_Domain=10Y1001A1001A82H&securityToken={}&periodStart={start_time}&periodEnd={end_time}", self.api_key);
        let resp : String = reqwest::blocking::get(req)?.text()?;
        println!("{:#?}", resp);
        Ok(resp)
    }

    pub fn query_day_ahead_prices(&self, start_time: &str, end_time: &str, area: &str) -> Result<String, Box<dyn Error>> {
        let mut params = HashMap::new();

        match Mappings::DOMAIN_MAPPINGS.get(&"DE_LU") {
            Some(&domain_value) => { 
                params.insert("in_Domain", domain_value);
                params.insert("out_Domain", domain_value); 
            },
            _ => println!("Don't have mapping for area."),
        }

        return self.basic_request(start_time, end_time, params);
    }

}