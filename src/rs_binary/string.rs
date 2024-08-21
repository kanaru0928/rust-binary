use super::base::{BinaryController, Codable, Decodable, Encodable, PointeredBinary};
use num_traits::{FromPrimitive, NumCast};

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
    fn to_binary(&self) -> PointeredBinary {
        let length: Option<T> = T::from_usize(self.data.len());
        let mut binary = length.unwrap().to_binary();
        for byte in self.data.as_bytes() {
            binary.write(vec![*byte]);
        }
        binary
    }
}

impl<T: Codable + NumCast + FromPrimitive> Decodable for DynamicString<T> {
    fn from_binary(data: &mut PointeredBinary) -> Self {
        let length = T::from_binary(data);
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

impl<T: Codable + NumCast + FromPrimitive> Codable for DynamicString<T> {}

pub struct DynamicStringBinaryController<T: Codable + NumCast + FromPrimitive> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Codable + NumCast + FromPrimitive> DynamicStringBinaryController<T> {
    pub fn new() -> Self {
        DynamicStringBinaryController {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Codable + NumCast + FromPrimitive> BinaryController<String>
    for DynamicStringBinaryController<T>
{
    fn encode(&self, data: String) -> PointeredBinary {
        let dynamic_string: DynamicString<T> = data.into();
        dynamic_string.to_binary()
    }

    fn decode(&self, data: &mut PointeredBinary) -> String {
        let dynamic_string = DynamicString::<T>::from_binary(data);
        dynamic_string.data
    }
}

pub struct SizedStringBinaryController {
    length: usize,
}

impl SizedStringBinaryController {
    pub fn new(length: usize) -> Self {
        SizedStringBinaryController { length }
    }
}

impl BinaryController<String> for SizedStringBinaryController {
    fn encode(&self, data: String) -> PointeredBinary {
        let mut binary = PointeredBinary::new(Vec::new());
        let mut bytes: Vec<u8> = data.as_bytes().to_vec();
        bytes.resize(self.length, 0);
        binary.write(bytes);
        binary
    }

    fn decode(&self, data: &mut PointeredBinary) -> String {
        let bytes = data.read(self.length).unwrap();
        let mut string = String::new();
        let mut length = 0;
        for byte in bytes.clone().into_iter() {
            if byte == 0 {
                break;
            }
            length += 1;
        }
        string.push_str(std::str::from_utf8(&bytes[0..length]).unwrap());
        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_string_encodable() {
        let string = "Hello, World!".to_string();
        let dynamic_string: DynamicString<u8> = string.into();
        let encoded = dynamic_string.to_binary();
        let data = encoded.get_data();
        assert_eq!(
            data,
            &[13, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]
        );
    }

    #[test]
    fn test_dynamic_string_decodable() {
        let data = vec![
            13, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33,
        ];
        let mut binary = PointeredBinary::new(data);
        let dynamic_string = DynamicString::<u8>::from_binary(&mut binary);
        assert_eq!(dynamic_string.data, "Hello, World!");
    }

    #[test]
    fn test_dynamic_u16_string_encodable() {
        let string = "Hello, World!".to_string();
        let dynamic_string: DynamicString<u16> = string.into();
        let encoded = dynamic_string.to_binary();
        let data = encoded.get_data();
        assert_eq!(
            data,
            &[13, 0, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]
        );
    }

    #[test]
    fn test_dynamic_u16_string_decodable() {
        let data = vec![
            13, 0, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33,
        ];
        let mut binary = PointeredBinary::new(data);
        let dynamic_string = DynamicString::<u16>::from_binary(&mut binary);
        assert_eq!(dynamic_string.data, "Hello, World!");
    }

    #[test]
    fn test_dynamic_u8_string_binary_controller() {
        let string = "Hello, World!".to_string();
        let controller = DynamicStringBinaryController::<u8>::new();
        let encoded = controller.encode(string.clone());
        let data = encoded.get_data();
        assert_eq!(
            data,
            &[13, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]
        );
        let mut binary = PointeredBinary::new(encoded.get_data().clone());

        let decoded = controller.decode(&mut binary);
        assert_eq!(decoded, string);
    }

    #[test]
    fn test_dynamic_u16_string_binary_controller() {
        let string = "Hello, World!".to_string();
        let controller = DynamicStringBinaryController::<u16>::new();
        let encoded = controller.encode(string.clone());
        let data: &Vec<u8> = encoded.get_data();
        assert_eq!(
            data,
            &[13, 0, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]
        );
        let mut binary = PointeredBinary::new(encoded.get_data().clone());

        let decoded = controller.decode(&mut binary);
        assert_eq!(decoded, string);
    }
    
    #[test]
    fn test_sized_string_encodable() {
        let string = "Hello, World!".to_string();
        let controller = SizedStringBinaryController::new(20);
        let encoded = controller.encode(string.clone());
        let data = encoded.get_data();
        assert_eq!(
            data,
            &[
                72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }
    
    #[test]
    fn test_sized_string_decodable() {
        let data = vec![
            72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut binary = PointeredBinary::new(data);
        let controller = SizedStringBinaryController::new(20);
        let decoded = controller.decode(&mut binary);
        assert_eq!(decoded, "Hello, World!");
    }
}
