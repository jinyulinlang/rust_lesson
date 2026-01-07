use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as char_parser, multispace0, none_of},
    combinator::{map, opt, value},
    multi::{many0, separated_list0},
    number::complete::recognize_float,
    sequence::{delimited, separated_pair},
};
use serde_json::Value;

fn main() {
    let input = r#"
    {
        "name": "John",
        "age": 30,
        "city": "New York",
        "is_active": true,
        "balance": 1000.50,
        "orders": [
            {
                "id": 1,
                "amount": 100,
                "status": "completed"
            },
            {
                "id": 2,
                "amount": 200,
                "status": "pending"
            }
        ],
        "created_at": "2021-01-01T00:00:00Z"
    }
    "#;

    let result = parse_primary(input);
    assert!(result.is_ok());
    let (_, value) = result.unwrap();
    println!("Parsed JSON: {:#?}", value);
}

fn parse_null(input: &str) -> IResult<&str, Value> {
    value(
        Value::Null,
        delimited(multispace0, tag("null"), multispace0),
    )
    .parse(input)
}

fn parse_bool(input: &str) -> IResult<&str, Value> {
    delimited(
        multispace0,
        alt((
            value(Value::Bool(true), tag("true")),
            value(Value::Bool(false), tag("false")),
        )),
        multispace0,
    )
    .parse(input)
}

fn parse_number(input: &str) -> IResult<&str, Value> {
    let (input, number) = delimited(multispace0, recognize_float, multispace0).parse(input)?;

    Ok((input, Value::Number(number.parse().unwrap())))
}

fn parse_string(input: &str) -> IResult<&str, Value> {
    map(
        delimited(multispace0, recognize_string, multispace0),
        |string| Value::String(string),
    )
    .parse(input)
}

fn recognize_string(input: &str) -> IResult<&str, String> {
    map(
        delimited(char_parser('"'), many0(none_of("\"")), char_parser('"')),
        |chars| chars.into_iter().collect(),
    )
    .parse(input)
}

fn parse_array(input: &str) -> IResult<&str, Value> {
    delimited(
        multispace0,
        delimited(
            (char_parser('['), opt(multispace0)),
            map(
                separated_list0(
                    delimited(multispace0, char_parser(','), multispace0),
                    parse_primary,
                ),
                |values| Value::Array(values),
            ),
            (char_parser(']'), opt(multispace0)),
        ),
        multispace0,
    )
    .parse(input)
}

fn parse_object(input: &str) -> IResult<&str, Value> {
    delimited(
        multispace0,
        delimited(
            (char_parser('{'), opt(multispace0)),
            map(
                separated_list0(
                    delimited(multispace0, char_parser(','), multispace0),
                    separated_pair(
                        recognize_string,
                        delimited(multispace0, char_parser(':'), multispace0),
                        parse_primary,
                    ),
                ),
                |entries| Value::Object(entries.into_iter().collect()),
            ),
            (char_parser('}'), opt(multispace0)),
        ),
        multispace0,
    )
    .parse(input)
}

fn parse_primary(input: &str) -> IResult<&str, Value> {
    alt((
        parse_null,
        parse_bool,
        parse_number,
        parse_string,
        parse_array,
        parse_object,
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_null() {
        let input = "null";
        let result = parse_null(input);
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        assert_eq!(value, Value::Null);
    }
    #[test]
    fn test_parse_bool() {
        let input = "true";
        let result = parse_bool(input);
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        assert_eq!(value, Value::Bool(true));
    }
    #[test]
    fn test_parse_number() {
        let input = "123.456";
        let result = parse_number(input);
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        assert_eq!(
            value,
            Value::Number(serde_json::Number::from_f64(123.456).unwrap())
        );
    }
    #[test]
    fn test_parse_string() {
        let input = "\"hello world\"";
        let result = parse_string(input);
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        assert_eq!(value, Value::String("hello world".to_string()));
    }
    #[test]
    fn test_parse_array() {
        let input = "[1, 2, 3]";
        let result = parse_array(input);
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        assert_eq!(
            value,
            Value::Array(vec![
                Value::Number(serde_json::Number::from(1)),
                Value::Number(serde_json::Number::from(2)),
                Value::Number(serde_json::Number::from(3))
            ])
        );
    }
    #[test]
    fn test_parse_object() {
        let input = "{\"name\": \"John\",      \"age\": 30}";
        let result = parse_object(input);
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        assert_eq!(
            value,
            Value::Object(
                vec![
                    ("name".to_string(), Value::String("John".to_string())),
                    (
                        "age".to_string(),
                        Value::Number(serde_json::Number::from(30))
                    )
                ]
                .into_iter()
                .collect()
            )
        );
    }
    #[test]
    fn test_parse_primary() {
        let input = "null";
        let result = parse_primary(input);
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        assert_eq!(value, Value::Null);
    }
}
