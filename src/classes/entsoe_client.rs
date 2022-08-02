use crate::classes::Mappings;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct EntsoeClient<'a> {
    pub api_key: &'a str,
}

pub struct DatetimeValue {
    pub dt: NaiveDateTime,
    pub val: f32,
}

impl<'a> EntsoeClient<'a> {
    pub fn basic_request(
        &self,
        start_time: &str,
        end_time: &str,
        params: HashMap<&str, &str>,
    ) -> Result<String, Box<dyn Error>> {
        // TODO build request with the parameters, not hard-coded
        let req : String = format!("https://transparency.entsoe.eu/api?documentType={}&in_Domain={}&out_Domain={}&securityToken={}&periodStart={}&periodEnd={}",
        params.get("documentType").unwrap(),params.get("in_Domain").unwrap(),
        params.get("out_Domain").unwrap(), self.api_key, start_time, end_time);
        let resp: String = reqwest::blocking::get(req)?.text()?;
        println!("{:#?}", resp);
        Ok(resp)
    }

    pub fn query_day_ahead_prices(
        &self,
        start_time: &str,
        end_time: &str,
        area: &str,
    ) -> Result<String, Box<dyn Error>> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("documentType", "A44");
        match Mappings::DOMAIN_MAPPINGS.get(area) {
            Some(&domain_value) => {
                params.insert("in_Domain", domain_value);
                params.insert("out_Domain", domain_value);
            }
            _ => println!("Don't have mapping for area."),
        }

        return self.basic_request(start_time, end_time, params);
    }
}
