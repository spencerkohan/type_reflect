use ts_quote::ts_string;

pub fn unit_case_validation(case_name: &str, type_name: &str) -> String {
    ts_string! {
        if (input === # "'#case_name'") {
            return input as #type_name
        }
    }
}
