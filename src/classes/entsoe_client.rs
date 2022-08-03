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

impl<'a> EntsoeClient<'a> {
    pub fn parse_xml_string(&self, xml_string: String) -> Vec<DatetimeValue> {
        let mut response_vector: Vec<DatetimeValue> = vec![];

        let mut reader = Reader::from_str(&xml_string[..]);
        reader.trim_text(true);

        let mut val_flag = false;
        let mut dt_flag = false;
        let mut buf = Vec::new();
        let mut actual_dt = NaiveDateTime::parse_from_str("197001010000", "%Y%m%d%H%M").unwrap(); // = NaiveDateTime::parse_from_str(START_TIME, "%Y%m%d%H%M").unwrap();
        let mut count = 0;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if let b"price.amount" = e.name() {
                        val_flag = true
                    } else if let b"start" = e.name() {
                        dt_flag = true
                    }
                }
                Ok(Event::Text(e)) => {
                    if val_flag {
                        response_vector.push(DatetimeValue {
                            dt: actual_dt + Duration::hours(count),
                            val: e
                                .unescape_and_decode(&reader)
                                .unwrap()
                                .parse::<f32>()
                                .unwrap(),
                        });
                        count += 1;
                        val_flag = false;
                    } else if dt_flag {
                        actual_dt = NaiveDateTime::parse_from_str(
                            &e.unescape_and_decode(&reader).unwrap()[..],
                            "%Y-%m-%dT%H:%MZ",
                        )
                        .unwrap();
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

    pub fn basic_request(
        &self,
        start_time: &str,
        end_time: &str,
        params: HashMap<&str, &str>,
    ) -> Result<Vec<DatetimeValue>, Box<dyn Error>> {
        let req : String = format!("https://transparency.entsoe.eu/api?documentType={}&in_Domain={}&out_Domain={}&securityToken={}&periodStart={}&periodEnd={}",
                    params.get("documentType").unwrap(),params.get("in_Domain").unwrap(),
                    params.get("out_Domain").unwrap(), self.api_key, start_time, end_time);
        let resp: String = reqwest::blocking::get(req)?.text()?;
        println!("{:#?}", resp);
        let res = self.parse_xml_string(resp);

        Ok(res)
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
}
