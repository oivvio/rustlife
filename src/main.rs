extern crate ncurses;

use ncurses::*;

use rand::Rng;

#[derive(Debug)]
struct Field {
    data: Vec<u8>,
    nrows: i32,
    ncols: i32,
}

impl Field {
    fn get_index(&self, row: i32, col: i32) -> usize {
        let mut row = row;
        let mut col = col;

        if row < 0 {
            row = self.nrows - 1;
        }

        if row >= self.nrows {
            row = 0;
        }

        if col < 0 {
            col = self.ncols - 1;
        }

        if col >= self.ncols {
            col = 0;
        }

        (col + (row * self.ncols)) as usize
    }

    fn print_field_diff(&self, other: &Field) {
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                let index = self.get_index(row, col);
                if self.data[index] != other.data[index] {
                    match other.data[index] {
                        0 => mvprintw(row as i32, col as i32, " "),
                        _ => mvprintw(row as i32, col as i32, "O"),
                    };
                }
            }
        }
        refresh();
    }

    fn count_neighbours(&self, row: i32, col: i32) -> u8 {
        let mut count: u8 = 0;

        count += self.data[self.get_index(row - 1, col - 1)];
        count += self.data[self.get_index(row - 1, col)];
        count += self.data[self.get_index(row - 1, col + 1)];

        count += self.data[self.get_index(row, col - 1)];
        count += self.data[self.get_index(row, col + 1)];

        count += self.data[self.get_index(row + 1, col - 1)];
        count += self.data[self.get_index(row + 1, col)];
        count += self.data[self.get_index(row + 1, col + 1)];

        count
    }

    fn update_field(&self) -> Field {
        // Init new empty field
        let mut new_field = Field::new(self.nrows, self.ncols);

        // Look att all positions in old field
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                // Count neighbours at this postion
                let count = self.count_neighbours(row, col);

                // Get value of current cell
                let current_cell = self.data[self.get_index(row, col)];

                // Use those to set new value
                new_field.data[self.get_index(row, col)] = match (current_cell, count) {
                    (1, 2) | (1, 3) | (0, 3) => 1,
                    _ => 0,
                };
            }
        }
        new_field
    }

    fn new(nrows: i32, ncols: i32) -> Field {
        Field {
            data: vec![0; (nrows * ncols) as usize],
            nrows: nrows,
            ncols: ncols,
        }
    }

    fn random_field(nrows: i32, ncols: i32) -> Field {
        let mut field = Field {
            data: vec![0; (nrows * ncols) as usize],
            nrows: nrows,
            ncols: ncols,
        };

        for row in 0..field.nrows {
            for col in 0..field.ncols {
                let index = field.get_index(row, col);
                field.data[index] = rand::thread_rng().gen_range(0, 2);
            }
        }

        field
    }
}

fn main() {
    let window = initscr();

    let ncols = (getmaxx(window) - 1) as i32;
    let nrows = (getmaxy(window) - 1) as i32;

    let mut field = Field::random_field(nrows, ncols);
    let mut new_field;

    let stop = 1000;
    // Don't wait for input
    // nodelay(window, true);
    let empty_field = Field::new(nrows, ncols);

    // print diff between first first field and empty
    empty_field.print_field_diff(&field);
    for _ in 1..stop {
        new_field = field.update_field();
        field.print_field_diff(&new_field);
        field = new_field;
    }

    printw("\nPress key\n");
    getch();
    endwin();
}
