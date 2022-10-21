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
    let args: Vec<String> = env::args().collect();
    join_all_but_first(args)
}

// Join all string elements of a vector into a single string but skip the first one
fn join_all_but_first(mut args: Vec<String>) -> String {
    let others: Vec<String> = args.drain(1..).collect();
    let query = others.join(" ");
    query
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

    #[test]
    fn test_join_all() {
        assert_eq!(
            "A B C",
            join_all_but_first(vec![
                "--".to_string(),
                "A".to_string(),
                "B".to_string(),
                "C".to_string()
            ])
        );
    }
}
