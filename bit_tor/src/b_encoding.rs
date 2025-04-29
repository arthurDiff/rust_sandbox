use std::{error::Error, slice::Iter};

const DICT_START: u8 = b'd';
const DICT_END: u8 = b'e';

const LIST_START: u8 = b'l';
const LIST_END: u8 = b'e';

const NUM_START: u8 = b'i';
const NUM_END: u8 = b'e';

const BYTE_ARR_DIVIDER: u8 = b':';

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct BEncoding {}

impl BEncoding {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let b_iter = bytes.iter();

        todo!()
    }

    pub fn decode_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        Self::decode(
            std::fs::read_to_string(path)
                .map_err(|err| Box::new(err))?
                .as_bytes(),
        )
    }

    // number decoder
    fn decode_number(b_iter: &mut Iter<u8>) -> Result<i128> {
        Ok(String::from_utf8_lossy(
            &b_iter
                .take_while(|v| **v != NUM_END)
                .cloned()
                .collect::<Vec<u8>>(),
        )
        .parse::<i128>()
        .map_err(|err| Box::new(err))?)
    }

    // string decoder
    fn decode_byte_arr(b_iter: &mut Iter<u8>) -> Result<String> {
        todo!()
    }
}
