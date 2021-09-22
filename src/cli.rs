pub struct StringChunks<'a> {
    slice: &'a str,
    step: usize,
}

impl<'a> StringChunks<'a> {
    pub fn new(slice: &'a str, step: usize) -> StringChunks<'a> {
        StringChunks { slice, step }
    }
}

impl<'a> Iterator for StringChunks<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        if self.slice.is_empty() {
            return None;
        }
        let (ret, rest) = self.slice.split_at(self.step);
        self.slice = rest;
        Some(ret)
    }
}
