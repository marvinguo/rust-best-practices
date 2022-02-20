struct Buffer<'a> {
    buf: &'a mut Vec<u8>,
    pos: usize,
}

impl<'a, 'b> Buffer<'a> {
    fn new(v: &'a mut Vec<u8>) -> Buffer {
        Buffer { buf: v, pos: 0 }
    }

    // we have 'a for buf
    // need another 'b for self
    fn read_bytes(&'b mut self) -> &'b [u8] {
        self.pos += 3;
        // self.buf.push(1);
        &self.buf[self.pos - 3..self.pos]
    }
}

fn print(b1: &[u8], b2: &[u8]) {
    println!("{:#?} {:#?}", b1, b2)
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let mut buf = Buffer::new(&mut v);

    let b1 = buf.read_bytes();
    let b2 = buf.read_bytes();
}