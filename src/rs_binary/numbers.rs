use super::base::{Decodable, Encodable, PointeredBinary};

macro_rules! impl_number_encodable {
    ($type:ty) => {
        impl Encodable for $type {
            fn encode(&self) -> PointeredBinary {
                let data = self.to_le_bytes().to_vec();
                PointeredBinary::new(data)
            }
        }
    };
}

macro_rules! impl_number_decodable {
    ($type:ty) => {
        impl Decodable for $type {
            fn decode(data: &mut PointeredBinary) -> Self {
                let bytes = data.read(std::mem::size_of::<Self>()).unwrap();
                let mut array = [0; std::mem::size_of::<Self>()];
                array.copy_from_slice(&bytes);
                Self::from_le_bytes(array)
            }
        }
    };
}

impl_number_encodable!(u8);
impl_number_decodable!(u8);
impl_number_encodable!(u16);
impl_number_decodable!(u16);
impl_number_encodable!(i32);
impl_number_decodable!(i32);
impl_number_encodable!(f32);
impl_number_decodable!(f32);
impl_number_encodable!(f64);
impl_number_decodable!(f64);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_u8_encodable() {
        let number: u8 = 0x12;
        let encoded = number.encode();
        let data = encoded.get_data();
        assert_eq!(data, &[0x12]);
    }
    
    #[test]
    fn test_u8_decodable() {
        let data = vec![0x12];
        let mut binary = PointeredBinary::new(data);
        let number = u8::decode(&mut binary);
        assert_eq!(number, 0x12);
    }

    #[test]
    fn test_u16_encodable() {
        let number: u16 = 0x1234;
        let encoded = number.encode();
        let data = encoded.get_data();
        assert_eq!(data, &[0x34, 0x12]);
    }

    #[test]
    fn test_u16_decodable() {
        let data = vec![0x34, 0x12];
        let mut binary = PointeredBinary::new(data);
        let number = u16::decode(&mut binary);
        assert_eq!(number, 0x1234);
    }
    
    #[test]
    fn test_i32_encodable() {
        let number: i32 = 0x12345678;
        let encoded = number.encode();
        let data = encoded.get_data();
        assert_eq!(data, &[0x78, 0x56, 0x34, 0x12]);
    }
    
    #[test]
    fn test_i32_decodable() {
        let data = vec![0x78, 0x56, 0x34, 0x12];
        let mut binary = PointeredBinary::new(data);
        let number = i32::decode(&mut binary);
        assert_eq!(number, 0x12345678);
    }
    
    #[test]
    fn test_f32_encodable() {
        let number: f32 = 3.14;
        let encoded = number.encode();
        let data = encoded.get_data();
        assert_eq!(data, &[0xc3, 0xf5, 0x48, 0x40]);
    }
    
    #[test]
    fn test_f32_decodable() {
        let data = vec![0xc3, 0xf5, 0x48, 0x40];
        let mut binary = PointeredBinary::new(data);
        let number = f32::decode(&mut binary);
        assert_eq!(number, 3.14);
    }
    
    #[test]
    fn test_f64_encodable() {
        let number: f64 = 3.14;
        let encoded = number.encode();
        let data = encoded.get_data();
        assert_eq!(data, &[0x1f, 0x85, 0xeb, 0x51, 0xb8, 0x1e, 0x09, 0x40]);
    }
    
    #[test]
    fn test_f64_decodable() {
        let data = vec![0x1f, 0x85, 0xeb, 0x51, 0xb8, 0x1e, 0x09, 0x40];
        let mut binary = PointeredBinary::new(data);
        let number = f64::decode(&mut binary);
        assert_eq!(number, 3.14);
    }
}
