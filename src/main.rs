use image::{Rgba, RgbaImage};
use std::fs;

// Простейшая 2D реализация "жадного меширования" для прямоугольных блоков (вокселей) в чанке.
// Результат сохраняется как output/chunk.png: фон — голубой, мешированные прямоугольники — зелёные.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Параметры чанка
    let chunk_w = 16usize;
    let chunk_h = 16usize;
    let tile_px = 16usize; // размер одного блока в пикселях

    let img_w = chunk_w * tile_px;
    let img_h = chunk_h * tile_px;

    // Создаём массив вокселей: true = блок присутствует, false = пусто.
    let mut voxels = vec![false; chunk_w * chunk_h];

    // Пример: большая прямоугольная область (трава) с несколькими пустотами
    for y in 4..12 {
        for x in 2..14 {
            voxels[y * chunk_w + x] = true;
        }
    }
    // Пара случайных дыр в области
    voxels[6 * chunk_w + 6] = false;
    voxels[9 * chunk_w + 10] = false;
    voxels[5 * chunk_w + 3] = false;

    // Жадное меширование (2D): объединяем соседние заполненные клетки в максимальные прямоугольники
    let mut visited = vec![false; chunk_w * chunk_h];
    let mut rects: Vec<(usize, usize, usize, usize)> = Vec::new(); // x,y,w,h in cells

    for y in 0..chunk_h {
        for x in 0..chunk_w {
            let idx = y * chunk_w + x;
            if visited[idx] || !voxels[idx] {
                continue;
            }

            // Определяем максимально возможную ширину
            let mut w = 1usize;
            while x + w < chunk_w && voxels[y * chunk_w + (x + w)] && !visited[y * chunk_w + (x + w)] {
                w += 1;
            }

            // Определяем высоту, расширяя вниз пока все клетки в ширине совпадают
            let mut h = 1usize;
            'expand_height: loop {
                if y + h >= chunk_h {
                    break;
                }
                for xx in x..x + w {
                    let idx2 = (y + h) * chunk_w + xx;
                    if !voxels[idx2] || visited[idx2] {
                        break 'expand_height;
                    }
                }
                h += 1;
            }

            // Отмечаем как посещённые
            for yy in y..y + h {
                for xx in x..x + w {
                    visited[yy * chunk_w + xx] = true;
                }
            }

            rects.push((x, y, w, h));
        }
    }

    // Подготавливаем изображение: фон — светло-голубой
    let bg = Rgba([135u8, 206u8, 250u8, 255u8]); // skyblue
    let green = Rgba([34u8, 139u8, 34u8, 255u8]); // forest green-ish

    let mut img = RgbaImage::from_pixel(img_w as u32, img_h as u32, bg);

    // Рисуем мешированные прямоугольники зелёным цветом
    for (x, y, w, h) in rects {
        let px0 = (x * tile_px) as u32;
        let py0 = (y * tile_px) as u32;
        let px1 = ((x + w) * tile_px) as u32;
        let py1 = ((y + h) * tile_px) as u32;

        for yy in py0..py1 {
            for xx in px0..px1 {
                img.put_pixel(xx, yy, green);
            }
        }
    }

    // Сохраняем результат
    fs::create_dir_all("output")?;
    let out_path = "output/chunk.png";
    img.save(out_path)?;

    println!("Saved chunk image with greedy meshing to {} ({}x{} px)", out_path, img_w, img_h);
    println!("Chunk cells: {}x{} ({} px per cell)", chunk_w, chunk_h, tile_px);

    Ok(())
}
