use num_traits::{FromPrimitive, NumCast};

use super::base::{
    BinaryController, Codable, Decodable, DefaultBinaryController, Encodable, PointeredBinary,
};

pub struct DynamicArrayBinaryController<
    T,
    BC: BinaryController<T>,
    U: NumCast + FromPrimitive + Codable,
> {
    _marker1: std::marker::PhantomData<T>,
    _marker2: std::marker::PhantomData<U>,
    controller: BC,
}

impl<T, BC: BinaryController<T>, U: NumCast + FromPrimitive + Codable>
    DynamicArrayBinaryController<T, BC, U>
{
    pub fn new(controller: BC) -> Self {
        DynamicArrayBinaryController {
            _marker1: std::marker::PhantomData,
            _marker2: std::marker::PhantomData,
            controller,
        }
    }
}

impl<T, BC: BinaryController<T>, U: NumCast + FromPrimitive + Codable> BinaryController<Vec<T>>
    for DynamicArrayBinaryController<T, BC, U>
{
    fn encode(&self, data: Vec<T>) -> PointeredBinary {
        let mut binary = PointeredBinary::new(Vec::new());
        let length = U::from_usize(data.len()).unwrap();
        binary.write(length.to_binary().get_data().clone());
        for item in data {
            binary.write(self.controller.encode(item).get_data().clone());
        }
        binary
    }

    fn decode(&self, data: &mut PointeredBinary) -> Vec<T> {
        let length = U::from_binary(data);
        let mut array = Vec::new();
        for _ in 0..length.to_usize().unwrap() {
            let item = self.controller.decode(data);
            array.push(item);
        }
        array
    }
}

pub struct DynamicArray<T: Codable, U: NumCast + FromPrimitive + Codable> {
    data: Vec<T>,
    _marker: std::marker::PhantomData<U>,
}

impl<T: Codable, U: NumCast + FromPrimitive + Codable> From<Vec<T>> for DynamicArray<T, U> {
    fn from(data: Vec<T>) -> Self {
        DynamicArray {
            data,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Codable + Clone, U: NumCast + FromPrimitive + Codable> Encodable for DynamicArray<T, U> {
    fn to_binary(&self) -> PointeredBinary {
        let controller = DefaultBinaryController::<T>::new();
        let binary_controller = DynamicArrayBinaryController::<T, _, U>::new(controller);
        binary_controller.encode(self.data.clone())
    }
}

impl<T: Codable, U: NumCast + FromPrimitive + Codable> Decodable for DynamicArray<T, U> {
    fn from_binary(data: &mut PointeredBinary) -> Self {
        let controller = DefaultBinaryController::<T>::new();
        let binary_controller = DynamicArrayBinaryController::<T, _, U>::new(controller);
        let vec = binary_controller.decode(data);
        DynamicArray {
            data: vec,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Codable + Clone, U: NumCast + FromPrimitive + Codable> Codable for DynamicArray<T, U> {}

pub struct SizedArrayBinaryController<T, BC: BinaryController<T>> {
    _marker1: std::marker::PhantomData<T>,
    controller: BC,
    length: usize,
}

impl<T, BC: BinaryController<T>> SizedArrayBinaryController<T, BC> {
    pub fn new(controller: BC, length: usize) -> Self {
        SizedArrayBinaryController {
            _marker1: std::marker::PhantomData,
            controller: controller,
            length: length,
        }
    }
}

impl<T: Clone, BC: BinaryController<T>> BinaryController<Vec<T>>
    for SizedArrayBinaryController<T, BC>
{
    fn encode(&self, data: Vec<T>) -> PointeredBinary {
        let mut binary = PointeredBinary::new(Vec::new());
        for _ in 0..self.length {
            binary.write(self.controller.encode(data[0].clone()).get_data().clone());
        }
        binary
    }

    fn decode(&self, data: &mut PointeredBinary) -> Vec<T> {
        let mut array = Vec::new();
        for _ in 0..self.length {
            let item = self.controller.decode(data);
            array.push(item);
        }
        array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_array_encodable() {
        let array: Vec<u8> = vec![0x12, 0x34, 0x56];
        let dynamic_array = DynamicArray::<u8, u8>::from(array);
        let encoded = dynamic_array.to_binary();
        let data = encoded.get_data();
        assert_eq!(data, &[0x03, 0x12, 0x34, 0x56]);
    }

    #[test]
    fn test_dynamic_array_decodable() {
        let data = vec![0x03, 0x12, 0x34, 0x56];
        let mut binary = PointeredBinary::new(data);
        let dynamic_array = DynamicArray::<u8, u8>::from_binary(&mut binary);
        assert_eq!(dynamic_array.data, vec![0x12, 0x34, 0x56]);
    }

    #[test]
    fn test_sized_array_binary_controller() {
        let controller = DefaultBinaryController::<u8>::new();
        let array: Vec<u8> = vec![0x12, 0x34, 0x56];
        let sized_array_controller = SizedArrayBinaryController::new(controller, 3);
        let encoded = sized_array_controller.encode(array);
        let data = encoded.get_data();
        assert_eq!(data, &[0x12, 0x12, 0x12]);
        
        let mut binary = PointeredBinary::new(data.to_vec());
        let decoded = sized_array_controller.decode(&mut binary);
        assert_eq!(decoded, vec![0x12, 0x12, 0x12]);
    }
}
