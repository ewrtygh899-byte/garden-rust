use macroquad::prelude::*;

const CHUNK_W: usize = 16;
const CHUNK_H: usize = 16;
const TILE_SIZE: f32 = 32.0; // pixels per cell

#[macroquad::main("Garden Rust - Chunk Viewer")]
async fn main() {
    // Создаём массив вокселей: true = блок присутствует, false = пусто.
    let mut voxels = vec![false; CHUNK_W * CHUNK_H];

    // Пример: большая прямоугольная область (трава) с несколькими пустотами
    for y in 4..12 {
        for x in 2..14 {
            voxels[y * CHUNK_W + x] = true;
        }
    }
    // Пара дыр
    voxels[6 * CHUNK_W + 6] = false;
    voxels[9 * CHUNK_W + 10] = false;
    voxels[5 * CHUNK_W + 3] = false;

    // Жадное меширование (2D): объединяем соседние заполненные клетки в максимальные прямоугольники
    let rects = greedy_meshing(&voxels, CHUNK_W, CHUNK_H);

    // Рассчитаем оконный размер так, чтобы весь чанк помещался
    let win_w = (CHUNK_W as f32) * TILE_SIZE;
    let win_h = (CHUNK_H as f32) * TILE_SIZE;

    // Установим удобный масштаб окна — macroquad задаёт размер в конфиге, но здесь мы можем просто масштабировать
    // Мы не настраиваем конфиг динамически, так что пользуемся размером экрана и центрируем чанк

    loop {
        // Фон — светло-голубой
        clear_background(Color::new(135.0 / 255.0, 206.0 / 255.0, 250.0 / 255.0, 1.0));

        // Позиция для отрисовки чанка: центрируем на экране
        let screen_w = screen_width();
        let screen_h = screen_height();
        let offset_x = (screen_w - win_w) / 2.0;
        let offset_y = (screen_h - win_h) / 2.0;

        // Рисуем границы чанка в виде рамки (опционально)
        draw_rectangle_lines(offset_x - 1.0, offset_y - 1.0, win_w + 2.0, win_h + 2.0, 2.0, BLACK);

        // Зеленый цвет для блоков
        let green = Color::new(34.0 / 255.0, 139.0 / 255.0, 34.0 / 255.0, 1.0);

        for (x, y, w, h) in &rects {
            let px = offset_x + (*x as f32) * TILE_SIZE;
            let py = offset_y + (*y as f32) * TILE_SIZE;
            let pw = (*w as f32) * TILE_SIZE;
            let ph = (*h as f32) * TILE_SIZE;

            draw_rectangle(px, py, pw, ph, green);
            // Нарисуем тонкую чёрную рамку вокруг каждого прямоугольника для наглядности
            draw_rectangle_lines(px, py, pw, ph, 2.0, BLACK);
        }

        // Подсказка
        draw_text(
            "Greedy meshing: green blocks on blue background. Press ESC to quit.",
            10.0,
            20.0,
            20.0,
            BLACK,
        );

        // Выход по ESC
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

fn greedy_meshing(voxels: &[bool], chunk_w: usize, chunk_h: usize) -> Vec<(usize, usize, usize, usize)> {
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

    rects
}
