use crate::world::chunk::Chunk;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

pub fn greedy_mesh(chunk: &Chunk) -> Vec<Rect> {
    let mut visited = vec![false; chunk.voxels.len()];
    let mut rects: Vec<Rect> = Vec::new();

    for y in 0..chunk.h {
        for x in 0..chunk.w {
            let idx = y * chunk.w + x;
            if visited[idx] || !chunk.get_voxel(x, y) {
                continue;
            }

            let mut w = 1usize;
            while x + w < chunk.w && chunk.get_voxel(x + w, y) && !visited[y * chunk.w + x + w] {
                w += 1;
            }

            let mut h = 1usize;
            'expand_height: loop {
                if y + h >= chunk.h {
                    break;
                }

                for xx in x..x + w {
                    let idx2 = (y + h) * chunk.w + xx;
                    if !chunk.get_voxel(xx, y + h) || visited[idx2] {
                        break 'expand_height;
                    }
                }

                h += 1;
            }

            for yy in y..y + h {
                for xx in x..x + w {
                    visited[yy * chunk.w + xx] = true;
                }
            }

            rects.push(Rect { x, y, w, h });
        }
    }

    rects
}