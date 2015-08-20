use std::string::String as std_String;
use std::io::Read;

pub struct String {
    pub data: std_String,
}

impl String {
    pub fn from_stream<R: Read>(stream : &mut R) -> Result<String, std_String> {
        let buf = [1u8];
        let string_size = match stream.read_to_end(&mut buf) {
            Ok(string_size) => string_size,
            Err(_) => return Err("Failed to read string length".to_string()),
        };

        // Read the string data
        let string_data_bytes = match stream.bytes(string_size as usize) {
            Ok(string_data_bytes) => string_data_bytes,
            Err(_) => return Err("Failed to read string data field".to_string()),
        };

        let data = match std_String::from_utf8(string_data_bytes) {
            Ok(data) => data,
            Err(_) => return Err("Failed to interpret string data as utf8 string".to_string()),
        };

        Ok(String {data: data})
    }
}

