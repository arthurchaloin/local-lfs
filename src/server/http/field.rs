use std::fmt;
use super::Error;

#[derive(Debug)]
pub struct Field {
    name:  String,
    value: String,
}

impl Field {
    pub fn new(name: String, value: String) -> Field {
        Field{ name, value }
    }

    pub fn new_contentlength(length: usize) -> Field {
        let name = String::from("Content-Length");
        let value = format!("{}", length);
        Field{ name, value }
    }

    pub fn from(line: String) -> Result<Field, Error> {
        // header-field   = field-name ":" OWS field-value OWS
        // field-value    = *( field-content / obs-fold )
        // field-content  = field-vchar [ 1*( SP / HTAB ) field-vchar ]
        // field-vchar    = VCHAR / obs-text
        // obs-fold       = CRLF 1*( SP / HTAB )
        //                ; obsolete line folding (see Section 3.2.4)
        // Split by the first colon separator.
        let sep = line.find(':');
        if sep.is_none() {
            return Error::err("Bad HTTP header");
        }
        // Parse name. Names must not contain whitespace.
        let name = String::from(&line[..sep.unwrap()]);
        if name.find(char::is_whitespace).is_some() {
            return Error::err("Bad HTTP header");
        }
        // Parse value. Values must have leading/trailing whitespace removed.
        // Line folding unsupported.
        let value = String::from(line[sep.unwrap()+1..].trim());
        if value.find('\n').is_some() {
            return Error::err("Bad HTTP header");
        }

        Ok(Field{ name, value })
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("{}: {}", self.name, self.value))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::tests::StringReader;

    fn assert_field_equals(name: &str, value: &str, actual: &Field) {
        assert_eq!(String::from(name), actual.name);
        assert_eq!(String::from(value), actual.value);
    }

    fn assert_parse_error(message: &str, result: Result<Field, Error>) {
        assert!(result.is_err());
        let description = format!("{}", result.unwrap_err());
        assert_eq!(message, description);
    }

    #[test]
    fn new() {
        let field = Field::new(String::from("foo"), String::from("bar"));
        assert_field_equals("foo", "bar", &field);
    }

    #[test]
    fn new_contentlength() {
        let field = Field::new_contentlength(42);
        assert_field_equals("Content-Length", "42", &field);
    }

    #[test]
    fn from_empty_string() {
        let result = Field::from(String::from(""));
        assert_parse_error("HTTP parsing error: Bad HTTP header", result);
    }

    #[test]
    fn from_invalid_name_string() {
        let result = Field::from(String::from("Invalid Foo: Bar"));
        assert_parse_error("HTTP parsing error: Bad HTTP header", result);
    }

    #[test]
    fn from_invalid_value_string() {
        let result = Field::from(String::from("Foo: Invalid\nBar"));
        assert_parse_error("HTTP parsing error: Bad HTTP header", result);
    }

    #[test]
    fn from_valid_string() {
        // Test without whitespace.
        let field = Field::from(String::from("Foo:Bar")).unwrap();
        assert_field_equals("Foo", "Bar", &field);

        // Test with whitespace which must be removed.
        let field = Field::from(String::from("Foo: \t bar bug \t ")).unwrap();
        assert_field_equals("Foo", "bar bug", &field);
    }

    #[test]
    fn display() {
        let field = Field::new(String::from("foo"), String::from("bar"));
        assert_eq!("foo: bar", format!("{}", field));
    }
}