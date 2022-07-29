use super::Element;
use super::Element::*;
use std::collections::HashMap;
use std::str::Chars;

impl Element {
    pub fn unmarshal(c: Chars) -> Result<Element, String> {
        Unmarshaler::new(c).unmarshal()
    }
}

pub struct Unmarshaler<'a> {
    chars: Chars<'a>,
    current: Option<char>,
    row: u32,
    col: u32,
}

impl<'a> Unmarshaler<'a> {
    pub fn new(c: Chars) -> Unmarshaler {
        Unmarshaler {
            chars: c,
            current: None,
            row: 1,
            col: 0,
        }
    }

    pub fn unmarshal(mut self) -> Result<Element, String> {
        self.next();
        let element = self.unmarshal_element()?;
        // Remaining chars should be whitespace
        self.skip_whitespace();
        if self.current != None {
            Err(self.unexpected())
        } else {
            Ok(element)
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current && c.is_whitespace() {
            self.next();
        }
    }

    fn eoc() -> String {
        "end of chars where element was expected".to_owned()
    }

    fn unexpected(&self) -> String {
        match self.current {
            None => Self::eoc(),
            Some(c) => format!("unexpected character `{}` at {}:{}", c, self.row, self.col),
        }
    }

    fn unmarshal_element(&mut self) -> Result<Element, String> {
        self.skip_whitespace();

        match self.current {
            Some('{') => self.unmarshal_object(),
            Some('[') => self.unmarshal_list(),
            Some('"') => self.unmarshal_string(),
            Some('n') => self.unmarshal_null(),
            Some('t' | 'f') => self.unmarshal_bool(),
            Some('-' | '0'..='9') => self.unmarshal_number(),
            _ => Err(self.unexpected()),
        }
    }

    fn unmarshal_object(&mut self) -> Result<Element, String> {
        if Some('{') != self.current {
            return Err(self.unexpected());
        }
        self.next();
        self.skip_whitespace();

        let mut elements = HashMap::new();
        if Some('}') == self.current {
            self.next();
            return Ok(JsonObject(elements));
        }

        // First key value pairs
        let (key, value) = self.unmarshal_key_value_pair()?;
        elements.insert(key, value);

        // Find any remaining key value pairs
        loop {
            self.skip_whitespace();
            match self.current {
                // End of object
                Some('}') => {
                    self.next();
                    return Ok(JsonObject(elements));
                }

                // Comma seperated key value pair
                Some(',') => {
                    self.next();
                    let (key, value) = self.unmarshal_key_value_pair()?;
                    elements.insert(key, value);
                }

                _ => return Err(self.unexpected()),
            }
        }
    }

    fn unmarshal_key_value_pair(&mut self) -> Result<(String, Element), String> {
        self.skip_whitespace();

        if Some('"') != self.current {
            return Err(self.unexpected());
        }

        let key = match read_escaped(self) {
            Ok(s) => s,
            _ => return Err(self.unexpected()),
        };
        self.next();

        self.skip_whitespace();
        if Some(':') != self.current {
            return Err(self.unexpected());
        }
        self.next();

        let value = self.unmarshal_element()?;

        Ok((key, value))
    }

    fn unmarshal_list(&mut self) -> Result<Element, String> {
        if Some('[') != self.current {
            return Err(self.unexpected());
        }
        self.next();
        self.skip_whitespace();

        match self.current {
            Some(']') => {
                // Empty list
                self.next();
                Ok(JsonList(vec![]))
            }
            Some(_) => {
                // List with at least one element
                let mut elements = vec![self.unmarshal_element()?];

                loop {
                    self.skip_whitespace();
                    match self.current {
                        Some(',') => {
                            // Add remaining elements
                            self.next();
                            elements.push(self.unmarshal_element()?)
                        }
                        Some(']') => {
                            self.next();
                            return Ok(JsonList(elements));
                        }
                        _ => return Err(self.unexpected()),
                    }
                }
            }
            _ => Err(self.unexpected()),
        }
    }

    fn unmarshal_string(&mut self) -> Result<Element, String> {
        match read_escaped(self) {
            Ok(s) => {
                self.next();
                Ok(JsonString(s))
            }
            _ => Err(self.unexpected()),
        }
    }

    fn unmarshal_number(&mut self) -> Result<Element, String> {
        let mut int = 0;

        let sign = match self.current {
            Some('-') => {
                // Negative sign must be followed by digit
                if let Some(d) = try_char_to_digit(self.next(), 10) {
                    int = d as i64;
                    self.next();
                } else {
                    return Err(self.unexpected());
                };
                -1
            }
            _ => 1,
        };

        while let Some(d) = try_char_to_digit(self.current, 10) {
            int = int * 10 + d as i64;
            self.next();
        }

        let json_num = if Some('.') == self.current {
            // Convert number to float if a decimal point is found
            self.next();
            if let Some(d) = try_char_to_digit(self.current, 10) {
                let mut float = int as f64 + d as f64 / 10.0;
                let mut decimal_place = 0.01;
                self.next();
                while let Some(d) = try_char_to_digit(self.current, 10) {
                    float += decimal_place * d as f64;
                    decimal_place /= 10.0;
                    self.next();
                }
                JsonFloat(sign as f64 * float)
            } else {
                // There must be a digit after a decimal point
                return Err(self.unexpected());
            }
        } else {
            JsonInt(sign * int)
        };

        match self.current {
            // Parse exponent if one exists
            Some('e' | 'E') => {
                let mut exponent = match self.next() {
                    Some('+') => 1.0,
                    Some('-') => -1.0,
                    c => {
                        if let Some(d) = try_char_to_digit(c, 10) {
                            d as f64
                        } else {
                            return Err(self.unexpected());
                        }
                    }
                };
                while let Some(d) = try_char_to_digit(self.next(), 10) {
                    exponent = exponent * 10.0 + d as f64;
                }
                match json_num {
                    JsonInt(int) => Ok(JsonFloat(int as f64 * 10f64.powf(exponent))),
                    JsonFloat(float) => Ok(JsonFloat(float * 10f64.powf(exponent))),
                    _ => Err(self.unexpected()), // Should never be reached
                }
            }
            _ => Ok(json_num),
        }
    }

    fn unmarshal_bool(&mut self) -> Result<Element, String> {
        match self.current {
            Some('t') => {
                if Some('r') == self.next() && Some('u') == self.next() && Some('e') == self.next()
                {
                    self.next();
                    Ok(JsonBool(true))
                } else {
                    Err(self.unexpected())
                }
            }
            Some('f') => {
                if Some('a') == self.next()
                    && Some('l') == self.next()
                    && Some('s') == self.next()
                    && Some('e') == self.next()
                {
                    self.next();
                    Ok(JsonBool(false))
                } else {
                    Err(self.unexpected())
                }
            }
            _ => Err(self.unexpected()),
        }
    }

    fn unmarshal_null(&mut self) -> Result<Element, String> {
        if Some('n') == self.current
            && Some('u') == self.next()
            && Some('l') == self.next()
            && Some('l') == self.next()
        {
            self.next();
            Ok(JsonNull)
        } else {
            Err(self.unexpected())
        }
    }
}

impl<'a> Iterator for Unmarshaler<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.current = self.chars.next();
        if let Some(c) = self.current {
            // Keep track of current row and column
            if c == '\n' {
                self.row += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        self.current
    }
}

// Read all characters into a string. Should not provide an opening quote, but
// does should have a closing quote at the end
fn read_escaped<T: Iterator<Item = char>>(chars: &mut T) -> Result<String, ReadEscapedErr> {
    let mut out = Vec::new();
    while let Some(c) = chars.next() {
        match c {
            // Expects a closing quote
            '"' => return Ok(out.into_iter().collect()),
            '\\' => match chars.next() {
                None => return Err(ReadEscapedErr::TrailingBackslash),
                Some(escaped @ ('\\' | '"' | '/')) => out.push(escaped),
                Some('b') => out.push('\x08'), // Literal backspace
                Some('f') => out.push('\x0c'), // Formfeed
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('t') => out.push('\t'),
                Some('u') => match try_read_unicode_escape(chars) {
                    Some(c) => out.push(c),
                    None => return Err(ReadEscapedErr::InvalidUnicodeEscape),
                },
                Some(unescapable) => {
                    return Err(ReadEscapedErr::InvalidBackslashEscape(unescapable))
                }
            },
            _ => out.push(c),
        }
    }
    Err(ReadEscapedErr::EndOfChars)
}

pub(crate) enum ReadEscapedErr {
    InvalidBackslashEscape(char),
    TrailingBackslash,
    InvalidUnicodeEscape,
    EndOfChars,
}

// Attempts to read unicode escape as "xxxx" or "xxxx\uxxxx" where x is a hex digit
fn try_read_unicode_escape<T: Iterator<Item = char>>(chars: &mut T) -> Option<char> {
    if let Some(h0) = try_char_to_digit(chars.next(), 16)
        && let Some(h1) = try_char_to_digit(chars.next(), 16)
        && let Some(h2) = try_char_to_digit(chars.next(), 16)
        && let Some(h3) = try_char_to_digit(chars.next(), 16)
    {
        let lead = (h0 << 12) + (h1 << 8) + (h2 << 4) + h3;
        if !(0xd800..=0xdbff).contains(&lead) {
            char::from_u32(lead)
        } else {
            // If lead is a surrogate, try to get the trailing value
            if Some('\\') != chars.next() || Some('u') != chars.next() {
                None
            } else if let Some(h0) = try_char_to_digit(chars.next(), 16)
                && let Some(h1) = try_char_to_digit(chars.next(), 16)
                && let Some(h2) = try_char_to_digit(chars.next(), 16)
                && let Some(h3) = try_char_to_digit(chars.next(), 16)
            {
                let trail = (h0 << 12) + (h1 << 8) + (h2 << 4) + h3;

                // This line is taken from the Python json lib
                let n = 0x10000 + (((lead - 0xd800) << 10) | (trail - 0xdc00));
                char::from_u32(n)
            } else {
                None
            }
        }
    } else {
        None
    }
}

fn try_char_to_digit(c: Option<char>, radix: u32) -> Option<u32> {
    if let Some(d) = c && let Some(n) = d.to_digit(radix) {
        Some(n)
    } else {
        None
    }
}
