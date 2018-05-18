pub struct Stream<T> {
    ary: Vec<T>,
    pos: usize,
}

impl<T> Stream<T> {
    pub fn new(ary: Vec<T>) -> Self {
        Self { ary: ary, pos: 0 }
    }
    pub fn get_pos(&self) -> usize {
        self.pos
    }

    pub fn peek(&self) -> Option<&T> {
        self.ary.get(self.pos)
    }

    pub fn skip(&mut self) {
        if self.is_exhausted() {
            panic!("Stream is exhausted");
        } else {
            self.pos += 1;
        }
    }

    pub fn next(&mut self) -> Option<&T> {
        let res = self.ary.get(self.pos);
        self.pos += 1;
        return res;
    }

    fn is_exhausted(&self) -> bool {
        self.ary.get(self.pos).is_none()
    }
}
