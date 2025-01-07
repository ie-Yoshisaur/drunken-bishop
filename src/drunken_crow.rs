use sha2::{Digest, Sha256};

pub fn generate_drunken_crow_grid(data: &[u8]) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let fingerprint = hasher.finalize();

    let rows = 3;
    let cols = 5;
    let mut grid = vec![vec![0u8; cols]; rows];

    let mut row = rows as i32 / 2;
    let mut col = cols as i32 / 2;
    let (start_row, start_col) = (row as usize, col as usize);

    for &byte in fingerprint.iter() {
        let mut b = byte;
        for _ in 0..4 {
            let dx = if (b & 0x01) != 0 { 1 } else { -1 };
            let dy = if (b & 0x02) != 0 { 1 } else { -1 };

            row += dy;
            col += dx;
            if row < 0 {
                row = 0;
            }
            if row >= rows as i32 {
                row = rows as i32 - 1;
            }
            if col < 0 {
                col = 0;
            }
            if col >= cols as i32 {
                col = cols as i32 - 1;
            }

            grid[row as usize][col as usize] = grid[row as usize][col as usize].saturating_add(1);

            b >>= 2;
        }
    }

    let (end_row, end_col) = (row as usize, col as usize);

    (grid, (start_row, start_col), (end_row, end_col))
}

pub fn render_drunken_crow_art(
    grid: &[Vec<u8>],
    start_pos: (usize, usize),
    end_pos: (usize, usize),
) -> String {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let (start_row, start_col) = start_pos;
    let (end_row, end_col) = end_pos;

    let symbols = [
        ' ', '.', 'o', '+', '=', '*', 'B', 'O', 'X', '@', '%', '#', '&', '8', '$', '0',
    ];

    let template = r#"オエーー!!　　　＿_
    　　　　 ＿＿_／　 ＼
    　　　／　 ／　／/⌒
    　　 / (ﾟ)/　／　/
    　　/　　(　/{}⌒{}{}
    　 ｜　　 ＼＼ﾟ{}{}{}
    ／　　 /⌒＼＼ﾟ{}{}{}
    /　　　｜　　　＼{}{}{}
    　　　 ｜　　　　　ﾞ{}
    　　　　　　　　　　　{}
    　　　　　　　　　　　{}"#;

    let mut symbols_replaced = Vec::with_capacity(rows * cols);
    for r in 0..rows {
        for c in 0..cols {
            if r == start_row && c == start_col {
                symbols_replaced.push('オ');
            } else if r == end_row && c == end_col {
                symbols_replaced.push('エ');
            } else {
                let count = grid[r][c] as usize;
                let symbol = if count < symbols.len() {
                    symbols[count]
                } else {
                    symbols[symbols.len() - 1]
                };
                symbols_replaced.push(symbol);
            }
        }
    }

    let mut result = template.to_string();
    for sym in symbols_replaced {
        if let Some(pos) = result.find("{}") {
            result.replace_range(pos..pos + 2, &sym.to_string());
        } else {
            break;
        }
    }

    result
}
