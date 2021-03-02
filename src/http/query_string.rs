use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc is an example query string.
// Some entries like "d" can have multiple values, that's why we have the Value enum.

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from (s: &'buf str) -> Self {
        let mut data = HashMap::new();
        
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..]
            }

            data
            .entry(key)
            .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        // vec![] is a special macro to initialize values in a vector
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    Value::Multiple(vec) => vec.push(val) 
                })
            .or_insert(Value::Single(val));
            
        }

        // Using the data here as a parameter to QueryString
        // the compiler can infer the type in the line let mut data = HashMap::new();
        QueryString {
            data
        }
    }
}

#[cfg(test)]
mod query_string_tests {
    use super::*;

    #[test]
    fn from_single() {
        let path = "a=1&b=2&c&d=&e===&d=7&d=abc";
        let query_string = QueryString::from(path);
        match query_string.get("a") {
            Some(value) => {
                assert_eq!(*value , Value::Single("1"));
            },
            None => {
                assert!(false);
            }
        }

        match query_string.get("b") {
            Some(value) => {
                assert_eq!(*value , Value::Single("2"));
            },
            None => {
                assert!(false);
            }
        }

        match query_string.get("c") {
            Some(value) => {
                assert_eq!(*value , Value::Single(""));
            },
            None => {
                assert!(false);
            }
        }

        match query_string.get("This should give me None") {
            Some(_) => {
                assert!(false);
            },
            None => {
                assert!(true);
            }
        }
    }

    #[test]
    fn from_multiple() {
        let path = "a=1&b=2&c&d=&e===&d=7&d=abc";
        let query_string = QueryString::from(path);
        match query_string.get("d") {
            Some(value) => {
                assert_eq!(*value , Value::Multiple(vec!["", "7", "abc"]));
            },
            None => {
                assert!(false);
            }
        }

        match query_string.get("This should give me None") {
            Some(_) => {
                assert!(false);
            },
            None => {
                assert!(true);
            }
        }
    }
}