use std::ops::Range;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let grid = Grid::build(&contents);
    println!("{:?}", grid);
    println!("Visible trees: {}", grid.get_visible());
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<u8>>,
}

impl Grid {
    fn build(data: &String) -> Self {
        let mut lines = data.lines();
        let width = lines.next().unwrap().len();
        let height = data.lines().count();
        let mut cells = Vec::with_capacity(height);
        for line in lines {
            let mut row = Vec::with_capacity(width);
            for c in line.chars() {
                row.push(c as u8);
            }
            cells.push(row);
        }
        return Grid {
            width,
            height,
            cells,
        };
    }

    fn check_column_visibility(&self, point: u8, column: usize, range: Range<usize>) -> bool {
        let mut is_visible = true;
        for down in range {
            if point <= self.cells[down][column] {
                is_visible = false;
                break;
            }
        }
        return is_visible;
    }

    fn check_row_visibility(&self, point: u8, row: usize, range: Range<usize>) -> bool {
        let mut is_visible = true;
        for right in range {
            if point <= self.cells[row][right] {
                is_visible = false;
                break;
            }
        }
        return is_visible;
    }

    fn get_visible(&self) -> u16 {
        let mut visible = 2 * (self.width + self.height - 2) as u16;

        for i in 1..self.height - 1 {
            for j in 1..self.width - 1 {
                let current_cell = self.cells[i][j];
                if self.check_row_visibility(current_cell, i, j..self.width) {
                    visible += 1;
                    continue;
                }
                if self.check_row_visibility(current_cell, i, 0..j) {
                    visible += 1;
                    continue;
                };

                if self.check_column_visibility(current_cell, j, i..self.height) {
                    visible += 1;
                    continue;
                }
                if self.check_column_visibility(current_cell, j, 0..i) {
                    visible += 1;
                    continue;
                }
            }
        }
        return visible;
    }
}
