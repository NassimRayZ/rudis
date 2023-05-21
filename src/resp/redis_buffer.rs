use super::parser::Parser;

pub struct RedisBuffer<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> RedisBuffer<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    fn next(&mut self) -> Option<&u8> {
        let byte = self.buf.get(self.pos)?;
        self.pos += 1;
        Some(byte)
    }

    pub fn read_u8(&mut self) -> Option<&u8> {
        self.next()
    }

    pub fn write_u8(&mut self, byte: u8) {
        self.buf[self.pos] = byte;
        self.pos += 1;
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.as_bytes() {
            self.write_u8(*byte);
        }
    }
    pub fn buf(&self) -> &[u8] {
        &self.buf
    }

    pub fn as_vec(&self) -> Vec<u8> {
        Vec::from(self.buf.to_owned())
    }
}

impl<'a> AsMut<RedisBuffer<'a>> for RedisBuffer<'a> {
    fn as_mut(&mut self) -> &mut RedisBuffer<'a> {
        self
    }
}

impl<'a, T> From<(&T, &'a mut [u8])> for RedisBuffer<'a>
where
    T: Parser<Self> + ToString,
{
    fn from(value: (&T, &'a mut [u8])) -> Self {
        let mut buffer = Self::new(value.1);
        buffer.write_string(&value.0.to_string());
        buffer
    }
}
