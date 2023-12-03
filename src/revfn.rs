pub struct RevFn {
    pub empty_boxes_fn: Vec<usize>
}

impl RevFn {

    pub fn set(&mut self, x: usize, y: usize) {
        self.empty_boxes_fn[y] = x;
    }
    pub fn reverse(&self, y: usize) -> usize {
        return self.empty_boxes_fn[y];
    }

    pub fn regular(&self, x: usize) -> usize {
        for (pos,val) in self.empty_boxes_fn.iter().enumerate().rev() {
            if *val <= x {
                return pos;
            }
        };
        return 0;
    }
}