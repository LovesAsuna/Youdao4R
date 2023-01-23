use crypto::digest::Digest;
use crypto::md5::Md5;
use rand::Rng;
use crate::translator::CLIENT;

const BV: &str = "1de9313c44872e4c200c577f99d4c09e";
const HEX_ARRAY: &[u8] = b"0123456789ABCDEF";

pub struct Credential {
    time: u128,
    salt: String,
    sign: String,
}

impl Credential {
    pub fn of(token: &str, translating_text: &str) -> Self {
        let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let salt = time.to_string() + &rand::thread_rng().gen_range(0..10).to_string();
        let msg = CLIENT.to_string() + translating_text + &salt + token;
        let mut digest = Md5::new();
        digest.input_str(&msg);
        let mut bytes = vec![0; (128 + 7) / 8];
        digest.result(&mut bytes);
        let sign = Self::bytes_to_hex(bytes).to_lowercase();
        Self {
            time,
            salt,
            sign,
        }
    }

    fn bytes_to_hex(bytes: Vec<u8>) -> String {
        let mut hex_chars = vec![0; bytes.len() * 2];
        for i in 0..bytes.len() {
            let v = (bytes[i] & 0xFF) as usize;
            hex_chars[i * 2] = HEX_ARRAY[v >> 4];
            hex_chars[i * 2 + 1] = HEX_ARRAY[v & 0x0F];
        }
        String::from_utf8(hex_chars).unwrap()
    }
}

impl Credential {
    pub fn to_params(&self) -> Vec<(&str, &str)> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("salt", &self.salt));
        params.push(("sign", &self.sign));
        params.push(("lts", Box::leak(Box::new(self.time.to_string()))));
        params.push(("bv", BV));
        params
    }
}