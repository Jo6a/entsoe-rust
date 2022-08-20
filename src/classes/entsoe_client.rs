use crate::classes::DatetimeValue;
use crate::classes::HelpFuncs;
use crate::classes::Mappings;
use std::collections::HashMap;
use std::error::Error;

pub struct EntsoeClient<'a> {
    pub api_key: &'a str,
}

impl<'a> EntsoeClient<'a> {
    pub fn basic_request(
        &self,
        start_time: &str,
        end_time: &str,
        params: HashMap<&str, &str>,
    ) -> Result<Vec<DatetimeValue>, Box<dyn Error>> {
        let mut uri_params: String = String::new();
        uri_params.push_str(&format!("securityToken={}", self.api_key)[..]);
        uri_params.push_str(&format!("&periodStart={}", start_time)[..]);
        uri_params.push_str(&format!("&periodEnd={}", end_time)[..]);
        for (key, value) in params {
            uri_params.push_str(&format!("&{key}={value}")[..]);
        }
        let req: String = format!("https://transparency.entsoe.eu/api?{}", uri_params);
        let resp: String = reqwest::blocking::get(req)?.text()?;
        println!("{:#?}", resp);
        let res_vec = HelpFuncs::parse_xml_string(&resp[..]);

        Ok(res_vec)
    }

    pub fn query_day_ahead_prices(
        &self,
        start_time: &str,
        end_time: &str,
        area: &str,
    ) -> Result<Vec<DatetimeValue>, Box<dyn Error>> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("documentType", "A44");
        match Mappings::DOMAIN_MAPPINGS.get(area) {
            Some(&domain_value) => {
                params.insert("in_Domain", domain_value);
                params.insert("out_Domain", domain_value);
            }
            _ => println!("Don't have mapping for area."),
        }
        self.basic_request(start_time, end_time, params)
    }

    pub fn query_net_position(
        &self,
        start_time: &str,
        end_time: &str,
        area: &str,
    ) -> Result<Vec<DatetimeValue>, Box<dyn Error>> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("documentType", "A25");
        params.insert("businessType", "B09");
        params.insert("Contract_MarketAgreement.Type", "A01");
        match Mappings::DOMAIN_MAPPINGS.get(area) {
            Some(&domain_value) => {
                params.insert("in_Domain", domain_value);
                params.insert("out_Domain", domain_value);
            }
            _ => println!("Don't have mapping for area."),
        }
        self.basic_request(start_time, end_time, params)
    }

    pub fn query_load(
        &self,
        start_time: &str,
        end_time: &str,
        area: &str,
    ) -> Result<Vec<DatetimeValue>, Box<dyn Error>> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("documentType", "A65");
        params.insert("processType", "A16");
        match Mappings::DOMAIN_MAPPINGS.get(area) {
            Some(&domain_value) => {
                params.insert("outBiddingZone_Domain", domain_value);
                params.insert("out_Domain", domain_value);
            }
            _ => println!("Don't have mapping for area."),
        }
        self.basic_request(start_time, end_time, params)
    }

    pub fn query_load_forecast(
        &self,
        start_time: &str,
        end_time: &str,
        area: &str,
        process_type: Option<&str>,
    ) -> Result<Vec<DatetimeValue>, Box<dyn Error>> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("documentType", "A65");
        params.insert("processType", process_type.unwrap_or("A01"));
        match Mappings::DOMAIN_MAPPINGS.get(area) {
            Some(&domain_value) => {
                params.insert("outBiddingZone_Domain", domain_value);
            }
            _ => println!("Don't have mapping for area."),
        }
        self.basic_request(start_time, end_time, params)
    }

    pub fn query_generation_forecast(
        &self,
        start_time: &str,
        end_time: &str,
        area: &str,
        process_type: Option<&str>,
    ) -> Result<Vec<DatetimeValue>, Box<dyn Error>> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("documentType", "A71");
        params.insert("processType", process_type.unwrap_or("A01"));
        match Mappings::DOMAIN_MAPPINGS.get(area) {
            Some(&domain_value) => {
                params.insert("outBiddingZone_Domain", domain_value);
            }
            _ => println!("Don't have mapping for area."),
        }
        self.basic_request(start_time, end_time, params)
    }

    pub fn query_wind_and_solar_forecast(
        &self,
        start_time: &str,
        end_time: &str,
        area: &str,
        process_type: Option<&str>,
    ) -> Result<Vec<DatetimeValue>, Box<dyn Error>> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("documentType", "A65");
        params.insert("processType", process_type.unwrap_or("A01"));
        match Mappings::DOMAIN_MAPPINGS.get(area) {
            Some(&domain_value) => {
                params.insert("outBiddingZone_Domain", domain_value);
            }
            _ => println!("Don't have mapping for area."),
        }
        self.basic_request(start_time, end_time, params)
    }

    pub fn query_generation(
        &self,
        start_time: &str,
        end_time: &str,
        area: &str,
    ) -> Result<Vec<DatetimeValue>, Box<dyn Error>> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("documentType", "A75");
        params.insert("processType", "A16");
        match Mappings::DOMAIN_MAPPINGS.get(area) {
            Some(&domain_value) => {
                params.insert("in_Domain", domain_value);
            }
            _ => println!("Don't have mapping for area."),
        }
        self.basic_request(start_time, end_time, params)
    }

    pub fn query_generation_per_plant(
        &self,
        start_time: &str,
        end_time: &str,
        area: &str,
    ) -> Result<Vec<DatetimeValue>, Box<dyn Error>> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("documentType", "A73");
        params.insert("processType", "A16");
        match Mappings::DOMAIN_MAPPINGS.get(area) {
            Some(&domain_value) => {
                params.insert("in_Domain", domain_value);
            }
            _ => println!("Don't have mapping for area."),
        }
        self.basic_request(start_time, end_time, params)
    }

    pub fn query_installed_generation_capacity(
        &self,
        start_time: &str,
        end_time: &str,
        area: &str,
    ) -> Result<Vec<DatetimeValue>, Box<dyn Error>> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("documentType", "A68");
        params.insert("processType", "A33");
        match Mappings::DOMAIN_MAPPINGS.get(area) {
            Some(&domain_value) => {
                params.insert("in_Domain", domain_value);
            }
            _ => println!("Don't have mapping for area."),
        }
        self.basic_request(start_time, end_time, params)
    }

    pub fn query_installed_generation_capacity_per_unit(
        &self,
        start_time: &str,
        end_time: &str,
        area: &str,
    ) -> Result<Vec<DatetimeValue>, Box<dyn Error>> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("documentType", "A71");
        params.insert("processType", "A33");
        match Mappings::DOMAIN_MAPPINGS.get(area) {
            Some(&domain_value) => {
                params.insert("in_Domain", domain_value);
            }
            _ => println!("Don't have mapping for area."),
        }
        self.basic_request(start_time, end_time, params)
    }

    pub fn query_aggregate_water_reservoirs_and_hydro_storage(
        &self,
        start_time: &str,
        end_time: &str,
        area: &str,
    ) -> Result<Vec<DatetimeValue>, Box<dyn Error>> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("documentType", "A72");
        params.insert("processType", "A16");
        match Mappings::DOMAIN_MAPPINGS.get(area) {
            Some(&domain_value) => {
                params.insert("in_Domain", domain_value);
            }
            _ => println!("Don't have mapping for area."),
        }
        self.basic_request(start_time, end_time, params)
    }
}
