use std::io::ErrorKind;

pub struct PointeredBinary {
    data: Vec<u8>,
    pointer: usize,
}

impl PointeredBinary {
    pub fn new(data: Vec<u8>) -> Self {
        PointeredBinary {
            data,
            pointer: 0,
        }
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
    fn encode(&self) -> PointeredBinary;
}

pub trait Decodable {
    fn decode(data: &mut PointeredBinary) -> Self;
}

pub trait Codable: Encodable + Decodable {}
