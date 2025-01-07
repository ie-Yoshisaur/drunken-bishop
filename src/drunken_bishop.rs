use sha2::{Digest, Sha256};

/// Generates a 2D grid of visitation counts using the Drunken Bishop algorithm.
/// Returns the grid, the start position, and the end position.
pub fn generate_drunken_bishop_grid(data: &[u8]) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    // 1. Compute the SHA-256 hash (fingerprint) of the input data.
    let mut hasher = Sha256::new();
    hasher.update(data);
    let fingerprint = hasher.finalize();

    // 2. Define the size of the grid.
    let rows = 9;
    let cols = 17;
    // 3. Create a 2D vector initialized to zero to track how many times each cell is visited.
    let mut grid = vec![vec![0u8; cols]; rows];

    // 4. Place the bishop in the center of the grid and record the starting position.
    let mut row = rows as i32 / 2;
    let mut col = cols as i32 / 2;
    let (start_row, start_col) = (row as usize, col as usize);

    // 5. Interpret each byte in the fingerprint as four moves (two bits per move).
    for &byte in fingerprint.iter() {
        let mut b = byte;
        for _ in 0..4 {
            // 5.1. Determine dx and dy based on the lowest two bits of b.
            let dx = if (b & 0x01) != 0 { 1 } else { -1 };
            let dy = if (b & 0x02) != 0 { 1 } else { -1 };

            // 5.2. Update position and clamp if it goes out of bounds.
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

            // 5.3. Increment the visit counter in the current cell.
            grid[row as usize][col as usize] = grid[row as usize][col as usize].saturating_add(1);

            // 5.4. Shift the byte to move to the next two bits.
            b >>= 2;
        }
    }

    // 6. Record the ending position of the bishop.
    let (end_row, end_col) = (row as usize, col as usize);

    (grid, (start_row, start_col), (end_row, end_col))
}

/// Renders a 2D grid produced by the Drunken Bishop algorithm as ASCII art.
pub fn render_drunken_bishop_art(
    grid: &[Vec<u8>],
    start_pos: (usize, usize),
    end_pos: (usize, usize),
) -> String {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let (start_row, start_col) = start_pos;
    let (end_row, end_col) = end_pos;

    // 1. Define the symbols corresponding to visit counts.
    let symbols = [
        ' ', '.', 'o', '+', '=', '*', 'B', 'O', 'X', '@', '%', '#', '&', '8', '$', '0',
    ];

    // 2. Construct ASCII art by building each row of text.
    let mut lines = vec![];

    // 2.1. Top border.
    let top_border = format!("+{}+", "-".repeat(cols));
    lines.push(top_border);

    // 2.2. Each row in the grid.
    for r in 0..rows {
        let mut row_string = String::new();
        row_string.push('|');
        for c in 0..cols {
            if r == start_row && c == start_col {
                // Mark start with 'S'
                row_string.push('S');
            } else if r == end_row && c == end_col {
                // Mark end with 'E'
                row_string.push('E');
            } else {
                // Map visit count to a symbol.
                let count = grid[r][c] as usize;
                let idx = if count >= symbols.len() {
                    symbols.len() - 1
                } else {
                    count
                };
                row_string.push(symbols[idx]);
            }
        }
        row_string.push('|');
        lines.push(row_string);
    }

    // 2.3. Bottom border.
    let bottom_border = format!("+{}+", "-".repeat(cols));
    lines.push(bottom_border);

    // 3. Join all lines into a single string.
    lines.join("\n")
}
