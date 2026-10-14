#[derive(Clone, Debug)]
pub struct Chunk {
    pub w: usize,
    pub h: usize,
    pub voxels: Vec<bool>,
}

impl Chunk {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            voxels: vec![false; w * h],
        }
    }

    pub fn set_voxel(&mut self, x: usize, y: usize, value: bool) {
        if x >= self.w || y >= self.h {
            return;
        }

        self.voxels[y * self.w + x] = value;
    }

    pub fn get_voxel(&self, x: usize, y: usize) -> bool {
        if x >= self.w || y >= self.h {
            return false;
        }

        self.voxels[y * self.w + x]
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, value: bool) {
        for yy in y..(y + h).min(self.h) {
            for xx in x..(x + w).min(self.w) {
                self.set_voxel(xx, yy, value);
            }
        }
    }
}