
#[cfg(test)]
mod tests {
    use chrono::{NaiveDateTime};
    use std::path::Path;
    use std::fs;

    use super::super::helper_funcs::{DatetimeValue, HelpFuncs};

    fn generate_data() -> Vec<DatetimeValue> {
        let mut resp: Vec<DatetimeValue> = vec![];
        for i in 0..10 {
            let some = DatetimeValue {
                dt: NaiveDateTime::from_timestamp(1_000_000_000 + i * 3600, 0),
                val: i as f64,
            };
            resp.push(some);
        }
        resp
    }

    #[test]
    fn write_results_csv_test() {
        let resp = generate_data();
        HelpFuncs::write_results_csv("results_test.csv", ";", resp);
        let res = Path::new("results_test.csv").exists();
        assert!(res);
        let _cleanup = fs::remove_file("results_test.csv");
    }

    #[test]
    fn write_results_json_test() {
        let resp = generate_data();
        HelpFuncs::write_results_json("results_test.json", resp);
        let res = Path::new("results_test.json").exists();
        assert!(res);
        let _cleanup = fs::remove_file("results_test.json");
    }

    #[test]
    fn plot_data_test() {
        let resp = generate_data();
        HelpFuncs::plot_data("results_chart_test.png", resp);
        let res = Path::new("results_chart_test.png").exists();
        assert!(res);
        let _cleanup = fs::remove_file("results_chart_test.png");
    }
}
