use std::default::Default;

pub struct Rope {
    len: usize,
    data: Option<String>,
    left: Option<Box<Rope>>,
    right: Option<Box<Rope>>,
}

impl Default for Rope {
    fn default() -> Self {
        Rope {
            len: 0,
            data: None,
            left: None,
            right: None,
        }
    }
}

impl From<String> for Rope {
    fn from(_s: String) -> Self {
        Default::default()
    }
}

impl Rope {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn as_string(&self) -> String {
        let mut s = String::with_capacity(self.len);
        if !self.is_empty() {
            if self.is_leaf() {
                let data = match self.data {
                    Some(ref d) => d,
                    None => panic!(""),
                };
                s.push_str(data);
            } else {
                let left = match self.left {
                    Some(ref l) => l,
                    None => panic!(""),
                };
                let left_str = left.as_string();
                s.push_str(&left_str);

                let right = match self.right {
                    Some(ref r) => r,
                    None => panic!(""),
                };
                let right_str = right.as_string();
                s.push_str(&right_str);
            }
        }
        s
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0 && self.data.is_none() && self.left.is_none() && self.right.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none() && self.data.is_some() && self.len > 0
    }

    pub fn is_not_leaf(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }

    pub fn length(&self) -> usize {
        self.len
    }

    pub fn join(self, rope: Rope) -> Rope {
        let len = self.len + rope.len;
        Rope {
            len: len,
            data: None,
            left: Some(Box::new(self)),
            right: Some(Box::new(rope)),
        }
    }

    pub fn char_at(&self, index: usize) -> char {
        if !self.is_empty() {
            let left = match self.left {
                Some(ref l) => l,
                None => panic!(""),
            };

            if index < left.len {
                left.char_at(index)
            } else {
                match self.right {
                    Some(ref r) => r.char_at(index),
                    None => panic!(""),
                }
            }
        } else {
            0 as char
        }
    }

    pub fn sub(&self) -> Self {
        Default::default()
    }
}
