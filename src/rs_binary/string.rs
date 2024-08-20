use num_traits::{FromPrimitive, NumCast};
use super::base::{Codable, Decodable, Encodable, PointeredBinary};

struct DynamicString<T: Codable + NumCast + FromPrimitive> {
    data: String,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Codable + NumCast + FromPrimitive> From<String> for DynamicString<T> {
    fn from(data: String) -> Self {
        DynamicString {
            data,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Codable + NumCast + FromPrimitive> Encodable for DynamicString<T> {
    fn encode(&self) -> PointeredBinary {
        let length: Option<T> = T::from_usize(self.data.len());
        let mut binary = length.unwrap().encode();
        for byte in self.data.as_bytes() {
            binary.write(vec![*byte]);
        }
        binary
    }
}

impl<T: Codable + NumCast + FromPrimitive> Decodable for DynamicString<T> {
    fn decode(data: &mut PointeredBinary) -> Self {
        let length = T::decode(data);
        let mut string = String::new();
        for _ in 0..length.to_usize().unwrap() {
            let byte = data.read(1).unwrap();
            string.push(byte[0] as char);
        }
        DynamicString {
            data: string,
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_string_encodable() {
        let string = "Hello, World!".to_string();
        let dynamic_string: DynamicString<u8> = string.into();
        let encoded = dynamic_string.encode();
        let data = encoded.get_data();
        assert_eq!(data, &[13, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]);
    }

    #[test]
    fn test_dynamic_string_decodable() {
        let data = vec![13, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33];
        let mut binary = PointeredBinary::new(data);
        let dynamic_string = DynamicString::<u8>::decode(&mut binary);
        assert_eq!(dynamic_string.data, "Hello, World!");
    }
    
    #[test]
    fn test_dynamic_u16_string_encodable() {
        let string = "Hello, World!".to_string();
        let dynamic_string: DynamicString<u16> = string.into();
        let encoded = dynamic_string.encode();
        let data = encoded.get_data();
        assert_eq!(data, &[13, 0, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]);
    }
    
    #[test]
    fn test_dynamic_u16_string_decodable() {
        let data = vec![13, 0, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33];
        let mut binary = PointeredBinary::new(data);
        let dynamic_string = DynamicString::<u16>::decode(&mut binary);
        assert_eq!(dynamic_string.data, "Hello, World!");
    }
}
