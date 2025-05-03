use indexmap::IndexMap;

use super::{super::Result, Value};

const DICT_START: u8 = b'd';
const DICT_END: u8 = b'e';

const ARRAY_START: u8 = b'l';
const ARRAY_END: u8 = b'e';

const NUM_START: u8 = b'i';
const NUM_END: u8 = b'e';

const STRING_DIVIDER: u8 = b':';

/* Example Encoding
d
  8:announce            33:http://192.168.1.74:6969/announce
  7:comment             17:Comment goes here
  10:created by         25:Transmission/2.92 (14714)
  13:creation date      i1460444420e
  8:encoding            5:UTF-
  4:info
    d
      6:length          i59616e
      4:name            9:lorem.txt
      12:piece length   i32768e
      6:pieces          40:L@fR���3�K*Ez�>_YS��86��"�&�p�<�6�C{�9G
      7:private         i0e
    e
e
*/

pub struct Encoder;

impl Encoder {
    pub fn encode(value: &Value) -> Result<Vec<u8>> {
        Self::encode_next(vec![], value)
    }

    pub fn encode_to_file<P: AsRef<std::path::Path>>(value: &Value, path: P) -> Result<Vec<u8>> {
        let bytes = Encoder::encode(value)?;

        _ = std::fs::write(path, &bytes)?;

        Ok(bytes)
    }

    fn encode_next(buf: Vec<u8>, value: &Value) -> Result<Vec<u8>> {
        match value {
            Value::Number(num_v) => Self::encode_number(buf, num_v),
            Value::String(str_v) => Self::encode_string(buf, str_v),
            Value::Array(arr) => Self::encode_array(buf, arr),
            Value::Object(dict) => Self::encode_dict(buf, dict),
            Value::Null => Ok(buf),
        }
    }

    fn encode_dict(mut buf: Vec<u8>, value: &IndexMap<String, Value>) -> Result<Vec<u8>> {
        buf.push(DICT_START);

        for (k, v) in value {
            buf.append(&mut k.as_bytes().iter().cloned().collect::<Vec<u8>>());
            buf = Self::encode_next(buf, v)?;
        }

        buf.push(DICT_END);
        Ok(buf)
    }

    fn encode_array(mut buf: Vec<u8>, values: &Vec<Value>) -> Result<Vec<u8>> {
        buf.push(ARRAY_START);
        for v in values {
            buf = Self::encode_next(buf, v)?;
        }
        buf.push(ARRAY_END);
        Ok(buf)
    }

    fn encode_number(mut buf: Vec<u8>, value: &i128) -> Result<Vec<u8>> {
        buf.push(NUM_START);
        buf.append(
            &mut value
                .to_string()
                .as_bytes()
                .iter()
                .cloned()
                .collect::<Vec<u8>>(),
        );
        buf.push(NUM_END);

        Ok(buf)
    }

    fn encode_string(mut buf: Vec<u8>, value: &str) -> Result<Vec<u8>> {
        let byte_len = value.len();
        buf.append(
            &mut byte_len
                .to_string()
                .as_bytes()
                .iter()
                .cloned()
                .collect::<Vec<u8>>(),
        );
        buf.push(STRING_DIVIDER);
        buf.append(&mut value.as_bytes().iter().cloned().collect::<Vec<u8>>());

        Ok(buf)
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;
}
