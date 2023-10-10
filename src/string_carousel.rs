pub struct StringCarousel {
    data:     Vec<String>,
    /// Insert position
    pos:      usize,
}

pub struct StringCarouselIter<'a> {
    string_carousel: &'a StringCarousel,
    idx: usize,
}

impl StringCarousel {
    pub fn init_with<F>(size: usize, init_func: F) -> Self
    where F: Fn() -> String {
        let mut data: Vec<String> = Vec::with_capacity(size);
        (0..size).for_each(|_| data.push(init_func()));
        Self {
            data,
            pos: 0,
        }
    }

    pub fn insert(&mut self, strings: &[&str]) {
        self.data[self.pos].clear();
        strings.iter().for_each(|string| self.data[self.pos].push_str(string));
        self.rotate();
    }
    
    pub fn rotate(&mut self) {
        self.pos = (self.pos + 1) % self.data.len();
    }

    pub fn clear_all(&mut self) {
        self.data.iter_mut().for_each(|s| s.clear());
    }
}

impl<'a> Iterator for StringCarouselIter<'a> {
    type Item = &'a str;
    /// Returns a slice of the next String if String is non-empty
    fn next(&mut self) -> Option<Self::Item> {
        let size = self.string_carousel.data.len();
        let pos  = self.string_carousel.pos;
        if self.idx < size {
            let data_idx = match pos < self.idx {
                true  => pos + (size - self.idx),
                false => pos - self.idx,
            };
            self.idx += 1;
            match &self.string_carousel.data[data_idx] {
                string if string.len() > 0 => Some(string),
                _ => None,  // THIS MIGHT CAUSE A BUG 
            }
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a StringCarousel {
    type Item = &'a str;
    type IntoIter = StringCarouselIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        StringCarouselIter {
            string_carousel: self,
            idx: 0,
        }
    }
}
