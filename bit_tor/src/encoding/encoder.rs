use indexmap::IndexMap;

use super::{super::Result, Value};

const DICT_START: u8 = b'd';
const DICT_END: u8 = b'e';

const ARRAY_START: u8 = b'l';
const ARRAY_END: u8 = b'e';

const NUM_START: u8 = b'i';
const NUM_END: u8 = b'e';

const BYTE_ARRAY_DIVIDER: u8 = b':';

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
            Value::Bytes(str_v) => Self::encode_byte_array(buf, str_v),
            Value::Array(arr) => Self::encode_array(buf, arr),
            Value::Object(dict) => Self::encode_dict(buf, dict),
            Value::Null => Ok(buf),
        }
    }

    fn encode_dict(mut buf: Vec<u8>, value: &IndexMap<String, Value>) -> Result<Vec<u8>> {
        buf.push(DICT_START);

        for (k, v) in value {
            // buf.append(&mut k.as_bytes().iter().cloned().collect::<Vec<u8>>());
            buf = Self::encode_byte_array(buf, k.as_bytes())?;
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

    fn encode_byte_array(mut buf: Vec<u8>, value: &[u8]) -> Result<Vec<u8>> {
        let byte_len = value.len();
        buf.append(
            &mut byte_len
                .to_string()
                .as_bytes()
                .iter()
                .cloned()
                .collect::<Vec<u8>>(),
        );
        buf.push(BYTE_ARRAY_DIVIDER);
        buf.append(&mut value.iter().cloned().collect::<Vec<u8>>());

        Ok(buf)
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn should_encode_value_collectly() {
        let sample_value = Value::Object(IndexMap::from([
            ("test1".into(), Value::Number(2783)),
            (
                "test2".into(),
                Value::Bytes(vec![116, 104, 105, 115, 105, 115, 116, 101, 115, 116]),
            ),
            (
                "test3".into(),
                Value::Object(IndexMap::from([(
                    "innertest".into(),
                    Value::Array(vec![
                        Value::Number(787),
                        Value::Bytes(vec![116, 104, 105, 115]),
                    ]),
                )])),
            ),
        ]));

        let encoded_bytes = Encoder::encode(&sample_value).unwrap();

        assert_eq!(
            String::from_utf8(encoded_bytes).unwrap(),
            "d5:test1i2783e5:test210:thisistest5:test3d9:innertestli787e4:thiseee"
        )
    }
}
