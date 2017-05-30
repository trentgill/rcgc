use json::Error;
use json::parse;
use json::JsonValue;

pub fn path_to_json(contents: &String) -> Result<JsonValue, String> {
    let json = parse(&contents).or_else( |err| {
        match err {
            Error::UnexpectedCharacter { ch, line, column } => {
                let error_message = format!("unexpected character {} at line {}, column {}", ch, line, column);
                return Err(error_message);
            }
            
            Error::UnexpectedEndOfJson => {
                return Err(String::from("unexpected end of JSON"));
            }
            
            Error::FailedUtf8Parsing => {
                return Err(String::from("failed UTF8 parsing"));
            }

            Error::WrongType(string) => {
                let error_message = format!("{} is the wrong type", string);
                return Err(error_message);
            }

            Error::ExceededDepthLimit => {
                return Err(String::from("exceeded depth limit"));
            }
        }

    });
    json
}
