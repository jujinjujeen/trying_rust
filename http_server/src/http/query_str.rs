use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
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
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for item in s.split('&') {
            let mut key = item;
            let mut val = "";
            if let Some(i) = item.find('=') {
                key = &item[..i];
                val = &item[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_value) => {
                        let vec = vec![prev_value, val];
                        *existing = Value::Multiple(vec);
                    },
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
                
        }
        QueryString { data }
    }
}
