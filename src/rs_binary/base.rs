use std::io::ErrorKind;

pub struct PointeredBinary {
    data: Vec<u8>,
    pointer: usize,
}

impl PointeredBinary {
    pub fn new(data: Vec<u8>) -> Self {
        PointeredBinary { data, pointer: 0 }
    }

    pub fn get_pointer(&self) -> usize {
        self.pointer
    }

    pub fn set_pointer(&mut self, pointer: usize) {
        self.pointer = pointer;
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn read(&mut self, size: usize) -> Result<Vec<u8>, ErrorKind> {
        if self.pointer + size > self.data.len() {
            return Err(ErrorKind::UnexpectedEof);
        }

        let mut data = Vec::new();
        for _ in 0..size {
            data.push(self.data[self.pointer]);
            self.pointer += 1;
        }

        Ok(data)
    }

    pub fn write(&mut self, data: Vec<u8>) {
        for byte in data {
            self.data.push(byte);
        }
    }
}

pub trait Encodable {
    fn to_binary(&self) -> PointeredBinary;
}

pub trait Decodable {
    fn from_binary(data: &mut PointeredBinary) -> Self;
}

pub trait Codable: Encodable + Decodable {}

pub trait BinaryController<T> {
    fn encode(&self, data: &T) -> PointeredBinary;
    fn decode(&self, data: &mut PointeredBinary) -> T;
}

pub struct DefaultBinaryController<T: Codable> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Codable> DefaultBinaryController<T> {
    pub fn new() -> Self {
        DefaultBinaryController {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Codable> BinaryController<T> for DefaultBinaryController<T> {
    fn encode(&self, data: &T) -> PointeredBinary {
        data.to_binary()
    }

    fn decode(&self, data: &mut PointeredBinary) -> T {
        T::from_binary(data)
    }
}
