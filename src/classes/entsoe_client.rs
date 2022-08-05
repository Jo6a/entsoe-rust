use crate::classes::Mappings;
use chrono::Duration;
use chrono::NaiveDateTime;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::error::Error;

pub struct EntsoeClient<'a> {
    pub api_key: &'a str,
}

pub struct DatetimeValue {
    pub dt: NaiveDateTime,
    pub val: f32,
}

pub fn parse_xml_string(xml_string: &str) -> Vec<DatetimeValue> {
    let mut response_vector: Vec<DatetimeValue> = vec![];

    let mut reader = Reader::from_str(xml_string);
    reader.trim_text(true);

    let mut val_flag = false;
    let mut dt_flag = false;
    let mut buf = Vec::new();
    let mut actual_dt = NaiveDateTime::parse_from_str("197001010000", "%Y%m%d%H%M").expect("actual_dt needs a valid default value");
    let mut count = 0;
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name() == b"price.amount" {
                    val_flag = true;
                } else if e.name() == b"start" {
                    dt_flag = true;
                }
            }
            Ok(Event::Text(e)) => {
                if val_flag {
                    response_vector.push(DatetimeValue {
                        dt: actual_dt + Duration::hours(count),
                        val: e
                            .unescape_and_decode(&reader)
                            .expect("Error decoding")
                            .parse::<f32>()
                            .expect("Parse error when converting string to float"),
                    });
                    count += 1;
                    val_flag = false;
                } else if dt_flag {
                    actual_dt = NaiveDateTime::parse_from_str(
                        &e.unescape_and_decode(&reader).expect("Error decoding")[..],
                        "%Y-%m-%dT%H:%MZ",
                    )
                    .expect("Error paring NaiveDateTime from string");
                    dt_flag = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    response_vector
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
        let res_vec = parse_xml_string(&resp[..]);

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
}
