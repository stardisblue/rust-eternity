use std::default::Default;

#[derive(Default)]
pub struct Crawler {
    pub step: Option<(u8, u8)>,
    size: u8,
}

impl Crawler {
    pub fn new(size: u8) -> Crawler {
        Crawler {
            size,
            ..Default::default()
        }
    }

    pub fn current(&self) -> Option<(u8, u8)> {
        self.step
    }

    pub fn next(&mut self) -> Option<(u8, u8)> {
        match self.step {
            Some((mut x, mut y)) => {
                y += 1;

                if y == self.size {
                    x += 1;
                    y = 0;
                }

                self.step = Some((x, y));
                if x == self.size {
                    self.step = None;
                }

                self.step
            }
            None => None,
        }
    }
}
