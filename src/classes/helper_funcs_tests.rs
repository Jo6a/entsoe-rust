
#[cfg(test)]
mod tests {
    use chrono::{NaiveDateTime};
    use std::path::Path;

    use super::super::helper_funcs::{DatetimeValue, HelpFuncs};

    #[test]
    fn write_results_json_test() {
        let mut resp: Vec<DatetimeValue> = vec![];
        for i in 0..100 {
            let some = DatetimeValue {
                dt: NaiveDateTime::from_timestamp(1_000_000_000 + i * 3600, 0),
                val: i as f64,
            };
            resp.push(some);
        }
        HelpFuncs::write_results_json("results_test.json", resp);
        let res = Path::new("results_test.json").exists();
        assert!(res);
    }
}
