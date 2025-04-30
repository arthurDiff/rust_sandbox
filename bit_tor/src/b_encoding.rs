use std::{collections::HashMap, error::Error, iter::Peekable, slice::Iter};

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

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(i128),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

pub struct BEncoding;

impl BEncoding {
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

        // rewrite this
        Ok(match **next_b {
            _ => Value::String(Self::decode_byte_arr(b_iter)?),
        })
    }

    // number decoder
    fn decode_number(b_iter: &mut Peekable<Iter<u8>>) -> Result<i128> {
        Ok(String::from_utf8_lossy(
            &b_iter
                .take_while(|v| **v != NUM_END)
                .cloned()
                .collect::<Vec<u8>>(),
        )
        .parse::<i128>()?)
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
