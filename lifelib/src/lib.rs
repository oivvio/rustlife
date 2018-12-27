#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate ncurses;
extern crate ndarray;
use ndarray::*;

use rand::Rng;

#[derive(Debug)]
struct Field {
    data: Vec<u8>,
    nrows: i32,
    ncols: i32,
}

struct NDField {
    data: Array2<u8>,
    nrows: i32,
    ncols: i32,
}

impl NDField {
    fn roll_over_row(&self, row: i32) -> usize {
        if row < 0 {
            return (self.nrows - 1) as usize;
        }

        if row >= self.nrows {
            return 0 as usize;
        }

        row as usize
    }

    fn roll_over_col(&self, col: i32) -> usize {
        if col < 0 {
            return (self.ncols - 1) as usize;
        }

        if col >= self.ncols {
            return 0 as usize;
        }

        col as usize
    }

    fn print_field_diff(&self, other: &NDField) {
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                let index = [row as usize, col as usize];
                if self.data[index] != other.data[index] {
                    match other.data[index] {
                        0 => ncurses::mvprintw(row as i32, col as i32, " "),
                        _ => ncurses::mvprintw(row as i32, col as i32, "O"),
                    };
                }
            }
        }
        ncurses::refresh();
    }

    fn count_neighbours(&self, row: i32, col: i32) -> u8 {
        self.data[[self.roll_over_row(row - 1), self.roll_over_col(col - 1)]]
            + self.data[[self.roll_over_row(row - 1), self.roll_over_col(col)]]
            + self.data[[self.roll_over_row(row - 1), self.roll_over_col(col + 1)]]
            + self.data[[self.roll_over_row(row), self.roll_over_col(col - 1)]]
            + self.data[[self.roll_over_row(row), self.roll_over_col(col + 1)]]
            + self.data[[self.roll_over_row(row + 1), self.roll_over_col(col - 1)]]
            + self.data[[self.roll_over_row(row + 1), self.roll_over_col(col)]]
            + self.data[[self.roll_over_row(row + 1), self.roll_over_col(col + 1)]]
    }

    fn count_neighbours_without_roll_over(&self, row: i32, col: i32) -> u8 {
        let mut count: u8 = 0;

        count += self.data[[(row - 1) as usize, (col - 1) as usize]];
        count += self.data[[(row - 1) as usize, (col) as usize]];
        count += self.data[[(row - 1) as usize, (col + 1) as usize]];

        count += self.data[[(row) as usize, (col - 1) as usize]];
        count += self.data[[(row) as usize, (col + 1) as usize]];

        count += self.data[[(row + 1) as usize, (col - 1) as usize]];
        count += self.data[[(row + 1) as usize, (col) as usize]];
        count += self.data[[(row + 1) as usize, (col + 1) as usize]];

        count
    }

    fn update_field(&self) -> NDField {
        // Init new empty field
        let mut new_field = NDField::new(self.nrows, self.ncols);

        // Look att all positions in old field
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                // Count neighbours at this postion
                let count = self.count_neighbours(row, col);

                // Get value of current cell
                let index = [row as usize, col as usize];

                let current_cell = self.data[index];

                // Use those to set new value
                //new_field.data[[self.roll_over_row(row), self.roll_over_col(col)]] = match (current_cell, count) {
                new_field.data[index] = match (current_cell, count) {
                    (1, 2) | (1, 3) | (0, 3) => 1,
                    _ => 0,
                };
            }
        }
        new_field
    }

    fn new(nrows: i32, ncols: i32) -> NDField {
        NDField {
            data: Array2::<u8>::zeros((nrows as usize, ncols as usize)),
            nrows: nrows,
            ncols: ncols,
        }
    }

    fn random_field(nrows: i32, ncols: i32) -> NDField {
        let mut field = NDField {
            data: Array2::<u8>::zeros((nrows as usize, ncols as usize)),
            nrows: nrows,
            ncols: ncols,
        };

        for row in 0..field.nrows {
            for col in 0..field.ncols {
                field.data[[row as usize, col as usize]] = rand::thread_rng().gen_range(0, 2);
            }
        }

        field
    }
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
                        0 => ncurses::mvprintw(row as i32, col as i32, " "),
                        _ => ncurses::mvprintw(row as i32, col as i32, "O"),
                    };
                }
            }
        }
        ncurses::refresh();
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

pub fn bench() {
    let ncols = 100 as i32;
    let nrows = 100 as i32;

    let field = Field::random_field(nrows, ncols);
    let mut _new_field;

    // Don't wait for input
    // nodelay(window, true);
    let _empty_field = Field::new(nrows, ncols);

    let stop = 20;

    // print diff between first first field and empty
    for _ in 1..stop {
        _new_field = field.update_field();
    }
}

pub fn ndbench() {
    let ncols = 100 as i32;
    let nrows = 100 as i32;

    let field = NDField::random_field(nrows, ncols);
    let mut _new_field;

    // Don't wait for input
    // nodelay(window, true);
    let _empty_field = NDField::new(nrows, ncols);

    let stop = 20;

    // print diff between first first field and empty
    for _ in 1..stop {
        _new_field = field.update_field();
    }
}

pub fn run() {
    let window = ncurses::initscr();

    let ncols = (ncurses::getmaxx(window) - 1) as i32;
    let nrows = (ncurses::getmaxy(window) - 1) as i32;

    let mut field = Field::random_field(nrows, ncols);
    let mut new_field;

    // Don't wait for input
    // nodelay(window, true);
    let empty_field = Field::new(nrows, ncols);

    let stop = 1000;

    // print diff between first first field and empty
    empty_field.print_field_diff(&field);
    for _ in 1..stop {
        new_field = field.update_field();
        field.print_field_diff(&new_field);
        field = new_field;
    }

    ncurses::printw("\nPress key\n");
    ncurses::getch();
    ncurses::endwin();
}

pub fn ndrun() {
    let window = ncurses::initscr();

    let ncols = (ncurses::getmaxx(window) - 1) as i32;
    let nrows = (ncurses::getmaxy(window) - 1) as i32;

    let mut field = NDField::random_field(nrows, ncols);
    let mut new_field;

    // Don't wait for input
    // nodelay(window, true);
    let empty_field = NDField::new(nrows, ncols);

    let stop = 1000;

    // print diff between first first field and empty
    empty_field.print_field_diff(&field);
    for _ in 1..stop {
        new_field = field.update_field();
        field.print_field_diff(&new_field);
        field = new_field;
    }

    ncurses::printw("\nPress key\n");
    ncurses::getch();
    ncurses::endwin();
}
