use chrono::Duration;
use chrono::NaiveDateTime;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs;

pub struct DatetimeValue {
    pub dt: NaiveDateTime,
    pub val: f64,
}

pub struct HelpFuncs {}

impl HelpFuncs {
    pub fn parse_xml_string(xml_string: &str) -> Vec<DatetimeValue> {
        let mut response_vector: Vec<DatetimeValue> = vec![];

        let mut reader = Reader::from_str(xml_string);
        reader.trim_text(true);

        let mut val_flag = false;
        let mut dt_flag = false;
        let mut buf = Vec::new();
        let mut actual_dt = NaiveDateTime::parse_from_str("197001010000", "%Y%m%d%H%M")
            .expect("actual_dt needs a valid default value");
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
                                .parse::<f64>()
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

    pub fn write_results_csv(filename: &str, separator: &str, result_vec: Vec<DatetimeValue>) {
        let mut content = String::new();
        content.push_str(&format!("datetime{separator} value\n")[..]);
        for v in result_vec {
            content.push_str(
                &format!(
                    "{}{separator} {}\n",
                    v.dt.format("%Y-%m-%d %H:%M:%S"),
                    v.val
                )[..],
            );
        }
        fs::write(filename, content).expect("Something went wrong writing the file");
    }
}
