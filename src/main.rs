use date_time_parser::DateParser;
use std::env;

const DEFAULT_DATE_FORMAT: &str = "%Y-%m-%d";

fn main() {
    let query = get_query();

    let date = text_to_date(&query).expect("date should have been parsable");

    println!("{}", date.format(DEFAULT_DATE_FORMAT))
}

/// Get the date query from command line arguments
fn get_query() -> String {
    let args: Vec<String> = env::args().skip(1).collect();
    args.join(" ")
}

// Converts a Natural Language date into a chrono::NaiveDate
fn text_to_date(query: &str) -> Option<chrono::NaiveDate> {
    let date = DateParser::parse(query);
    date
}

#[cfg(test)]
mod tests {

    use chrono::offset::Local;
    use chrono::Duration;

    use super::*;

    #[test]
    fn test_natural_language() {
        assert_eq!(text_to_date("today"), Some(Local::now().date_naive()));
        assert_eq!(
            text_to_date("tomorrow"),
            Some(Local::now().date_naive() + Duration::days(1))
        );
    }
}
