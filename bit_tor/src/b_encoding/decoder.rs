use std::{iter::Peekable, slice::Iter};

use super::{super::Result, Value};
use indexmap::IndexMap;

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

pub struct Decoder;

impl Decoder {
    pub fn decode(bytes: &[u8]) -> Result<Value> {
        let mut b_iters = bytes.iter().peekable();
        Self::decode_next(&mut b_iters)
    }

    pub fn decode_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Value> {
        Self::decode(
            std::fs::read_to_string(path)
                .map_err(|err| Box::new(err))?
                .as_bytes(),
        )
    }

    pub fn decode_next(b_iter: &mut Peekable<Iter<u8>>) -> Result<Value> {
        let Some(next_b) = b_iter.peek() else {
            return Ok(Value::Null);
        };

        Ok(match **next_b {
            DICT_START => Value::Object(Self::decode_dict(b_iter)?),
            ARRAY_START => Value::Array(Self::decode_array(b_iter)?),
            NUM_START => Value::Number(Self::decode_number(b_iter)?),
            _ => Value::String(Self::decode_string(b_iter)?),
        })
    }

    fn decode_dict(b_iter: &mut Peekable<Iter<u8>>) -> Result<IndexMap<String, Value>> {
        // move over DICT_START
        _ = b_iter.next();

        let mut idx_map = IndexMap::new();
        while let Some(b) = b_iter.peek() {
            if **b == DICT_END {
                // move over DICT_END
                _ = b_iter.next();
                break;
            }

            idx_map.insert(Self::decode_string(b_iter)?, Self::decode_next(b_iter)?);
        }

        Ok(idx_map)
    }

    fn decode_array(b_iter: &mut Peekable<Iter<u8>>) -> Result<Vec<Value>> {
        // move over ARRAY_START
        _ = b_iter.next();

        let mut list = vec![];
        while let Some(b) = b_iter.peek() {
            if **b == ARRAY_END {
                // move over ARRAY_END
                _ = b_iter.next();
                break;
            }

            list.push(Self::decode_next(b_iter)?);
        }

        Ok(list)
    }

    // number decoder
    fn decode_number(b_iter: &mut Peekable<Iter<u8>>) -> Result<i128> {
        // move over NUM_START
        _ = b_iter.next();

        let n = String::from_utf8_lossy(
            &b_iter
                .take_while(|v| **v != NUM_END)
                .cloned()
                .collect::<Vec<u8>>(),
        )
        .parse::<i128>()?;

        // move over NUM_END
        _ = b_iter.next();

        Ok(n)
    }

    // string decoder
    fn decode_string(b_iter: &mut Peekable<Iter<u8>>) -> Result<String> {
        let b_len = String::from_utf8(
            b_iter
                .take_while(|v| **v != STRING_DIVIDER)
                .cloned()
                .collect(),
        )?
        .parse::<usize>()?;

        // move over ':'
        _ = b_iter.next();

        Ok(String::from_utf8(b_iter.take(b_len).cloned().collect())?)
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn should_decode_b_encoded_bytes_correctly() {
        let sample_encoded_bytes = r#"d8:announce33:http://192.168.1.74:6969/announce7:comment17:Comment goes here10:created by25:Transmission/2.92 (14714)13:creation datei1460444420e8:encoding5:UTF-84:infod6:lengthi59616e4:name9:lorem.txt12:piece lengthi32768e6:pieces40:L@fR���3�K*Ez�>_YS��86��"�&�p�<�6�C{�9G7:privatei0eee"#.as_bytes();

        let value = Decoder::decode(sample_encoded_bytes).unwrap();

        assert!(matches!(value, Value::Object(_)));
    }
}
