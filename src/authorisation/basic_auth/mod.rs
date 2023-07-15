use std::ops::Deref;
use regex::Regex;
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

#[derive(PartialEq, Debug)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    pub fn from_authorisation_header(header: &str) -> Option<BasicAuth> {
        let re = Regex::new(r"^[b|B]asic ").unwrap();

        if !re.is_match(header) {
            return None;
        }

        let binding = re.replace(header, "");
        let user_data = Self::decode_base64_auth_header(&(binding.deref()))?;

        Some(BasicAuth {
            username: user_data.0,
            password: user_data.1,
        })
    }

    fn decode_base64_auth_header(auth: &str) -> Option<(String, String)> {
        if let Err(error) = general_purpose::STANDARD.decode(auth.trim()) {
            return None;
        }
        let decoded = general_purpose::STANDARD.decode(auth.trim()).ok().unwrap();
        let decoded_str = String::from_utf8(decoded).ok().unwrap();
        let user_values: Vec<&str> = decoded_str.split(":").collect();
        if user_values.len() != 2 {
            return None;
        }
        Some((user_values[0].to_string(), user_values[1].to_string()))
    }
}

#[cfg(test)]
mod tests {
    use std::any::type_name;
    use crate::authorisation::basic_auth::{*};
    use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

    const USERNAME: &str = "john@doe.net";
    const PASSWORD: &str = "123456";

    #[test]
    fn should_return_a_valid_basic_auth_from_header() {
        let encoded_header = general_purpose::STANDARD.encode(format!("{}:{}", USERNAME, PASSWORD));
        let actual = BasicAuth::from_authorisation_header(&format!("{} {}", "Basic".to_string(), encoded_header)).unwrap();
        let expected = BasicAuth {
            username: USERNAME.to_string(),
            password: PASSWORD.to_string(),
        };
        assert_eq!(actual, expected)
    }

    #[test]
    fn should_return_none_when_the_header_is_empty() {
        let actual = BasicAuth::from_authorisation_header(&format!(""));
        let expected = None;
        assert_eq!(actual, expected)
    }

    #[test]
    fn should_return_none_when_the_header_is_not_properly_formatted() {
        let actual = BasicAuth::from_authorisation_header(&format!("sadsda"));
        let expected = None;
        assert_eq!(actual, expected)
    }

    #[test]
    fn should_return_a_expected_basic_auth_from_header() {
        let encoded_header = general_purpose::STANDARD.encode(format!("{}:{}", "palomo@usuriaga.com", "open_sesame"));
        let actual = BasicAuth::from_authorisation_header(&format!("{} {}", "basic".to_string(), encoded_header)).unwrap();
        let expected = BasicAuth {
            username: "palomo@usuriaga.com".to_string(),
            password: "open_sesame".to_string(),
        };
        assert_eq!(actual, expected)
    }

    #[test]
    fn should_return_none_when_the_encoded_string_is_not_well_formatted() {
        let encoded_header = general_purpose::STANDARD.encode(format!("{}{}", "blah", "open_sesame"));
        let actual = BasicAuth::from_authorisation_header(&format!("{} {}", "basic".to_string(), encoded_header));
        let expected = None;
        assert_eq!(actual, expected)
    }
}