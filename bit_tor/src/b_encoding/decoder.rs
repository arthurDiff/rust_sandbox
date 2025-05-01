use std::{error::Error, iter::Peekable, slice::Iter};

use super::Value;
use indexmap::IndexMap;

const DICT_START: u8 = b'd';
const DICT_END: u8 = b'e';

const LIST_START: u8 = b'l';
const LIST_END: u8 = b'e';

const NUM_START: u8 = b'i';
const NUM_END: u8 = b'e';

const BYTE_ARR_DIVIDER: u8 = b':';

type Result<T> = std::result::Result<T, Box<dyn Error>>;

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
            LIST_START => Value::List(Self::decode_list(b_iter)?),
            NUM_START => Value::Number(Self::decode_number(b_iter)?),
            _ => Value::String(Self::decode_byte_arr(b_iter)?),
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

            idx_map.insert(Self::decode_byte_arr(b_iter)?, Self::decode_next(b_iter)?);
        }

        Ok(idx_map)
    }

    fn decode_list(b_iter: &mut Peekable<Iter<u8>>) -> Result<Vec<Value>> {
        // move over LIST_START
        _ = b_iter.next();

        let mut list = vec![];
        while let Some(b) = b_iter.peek() {
            if **b == LIST_END {
                // move over LIST_END
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
    fn decode_byte_arr(b_iter: &mut Peekable<Iter<u8>>) -> Result<String> {
        let b_len = String::from_utf8(
            b_iter
                .take_while(|v| **v != BYTE_ARR_DIVIDER)
                .cloned()
                .collect(),
        )?
        .parse::<usize>()?;

        // move over ':'
        _ = b_iter.next();

        Ok(String::from_utf8(b_iter.take(b_len).cloned().collect())?)
    }
}
