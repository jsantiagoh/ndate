use date_time_parser::DateParser;
use std::env;
use text_colorizer::*;

const DEFAULT_DATE_FORMAT: &str = "%Y-%m-%d";
const ERROR_ARGUMENTS: i32 = 1;
const ERROR_PARSE: i32 = 2;

fn main() {
    let query = parse_args();

    match text_to_date(&query) {
        Ok(date) => println!("{}", date.format(DEFAULT_DATE_FORMAT)),
        Err(_) => std::process::exit(ERROR_PARSE),
    }
}

fn print_usage() {
    eprintln!(
        "{} - Convert a Natural language date query into a concrete date.",
        "ndate".green()
    );
    eprintln!("Usage: ndate <query>");
}

/// Parse Arguments and extract the query date
fn parse_args() -> String {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        print_usage();
        std::process::exit(ERROR_ARGUMENTS);
    }

    args.join(" ")
}

// Converts a Natural Language date into a chrono::NaiveDate
fn text_to_date(query: &str) -> Result<chrono::NaiveDate, String> {
    DateParser::parse(query).ok_or(format!("Unable to parse query: {query}"))
}

#[cfg(test)]
mod tests {

    use chrono::offset::Local;
    use chrono::Duration;

    use super::*;

    #[test]
    fn test_natural_language() {
        assert_eq!(text_to_date("today"), Ok(Local::now().date_naive()));
        assert_eq!(
            text_to_date("tomorrow"),
            Ok(Local::now().date_naive() + Duration::days(1))
        );
    }
}
