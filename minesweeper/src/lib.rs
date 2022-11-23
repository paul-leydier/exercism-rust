pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }
    let count = count_mines(minefield);
    format_count(minefield, count)
}

fn count_mines(minefield: &[&str]) -> Vec<Vec<u8>> {
    let height = minefield.len();
    let width = minefield[0].len();
    let mut count = vec![vec![0u8; width]; height];
    for (i, row) in minefield.iter().enumerate() {
        for (j, c) in row.bytes().enumerate() {
            if c != b'*' {
                continue;
            }
            if i > 0 {
                if j > 0 {
                    count[i - 1][j - 1] += 1;
                }
                count[i - 1][j] += 1;
                if j < width - 1 {
                    count[i - 1][j + 1] += 1;
                }
            }
            if j > 0 {
                count[i][j - 1] += 1;
            }
            if j < width - 1 {
                count[i][j + 1] += 1;
            }
            if i < height - 1 {
                if j > 0 {
                    count[i + 1][j - 1] += 1;
                }
                count[i + 1][j] += 1;
                if j < width - 1 {
                    count[i + 1][j + 1] += 1;
                }
            }
        }
    }
    count
}

fn format_count(minefield: &[&str], count: Vec<Vec<u8>>) -> Vec<String> {
    let mut result = Vec::new();
    for (i, row) in minefield.iter().enumerate() {
        let mut row_str = String::new();
        for (j, c) in row.bytes().enumerate() {
            if c == b'*' {
                row_str.push('*');
                continue;
            }
            if count[i][j] == 0 {
                row_str.push(' ');
                continue;
            }
            row_str.push_str(&format!("{}", count[i][j]));
        }
        result.push(row_str);
    }
    result
}
