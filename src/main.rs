extern crate ncurses;

use ncurses::*;

use rand::Rng;

struct Field {
    data: Vec<u8>,
    nrows: i16,
    ncols: i16,
}

fn print_field_diff(old_field: &Field, new_field: &Field) {
    printw("\n");

    for row in 0..old_field.nrows {
        for col in 0..old_field.ncols {
            let index = get_index(&old_field, row, col);
            if old_field.data[index] != new_field.data[index] {
                match new_field.data[index] {
                    0 => mvprintw(row as i32, col as i32, " "),
                    _ => mvprintw(row as i32, col as i32, "X"),
                };
            }
        }
        printw("\n");
    }
    refresh();
}

fn get_new_field(nrows: i16, ncols: i16) -> Field {
    let field = Field {
        data: vec![0; (nrows * ncols) as usize],
        nrows: nrows,
        ncols: ncols,
    };
    return field;
}

fn get_new_random_field(nrows: i16, ncols: i16) -> Field {
    let mut field = Field {
        data: vec![0; (nrows * ncols) as usize],
        nrows: nrows,
        ncols: ncols,
    };

    for row in 0..field.nrows {
        for col in 0..field.ncols {
            let index = get_index(&field, row, col);
            field.data[index] = rand::thread_rng().gen_range(0, 2);
        }
    }

    field
}

fn get_index(field: &Field, row: i16, col: i16) -> usize {
    let mut row = row;
    let mut col = col;

    if row < 0 {
        row = field.nrows - 1;
    }

    if row >= field.nrows {
        row = 0;
    }

    if col < 0 {
        col = field.ncols - 1;
    }

    if col >= field.ncols {
        col = 0;
    }

    (col + (row * field.ncols)) as usize
}

fn count_neighbours(field: &Field, row: i16, col: i16) -> u8 {
    let mut count: u8 = 0;

    count += field.data[get_index(&field, row - 1, col - 1)];
    count += field.data[get_index(&field, row - 1, col)];
    count += field.data[get_index(&field, row - 1, col + 1)];

    count += field.data[get_index(&field, row, col - 1)];
    count += field.data[get_index(&field, row, col + 1)];

    count += field.data[get_index(&field, row + 1, col - 1)];
    count += field.data[get_index(&field, row + 1, col)];
    count += field.data[get_index(&field, row + 1, col + 1)];

    count
}

fn update_field(field: &Field) -> Field {
    // Init new empty field
    let mut new_field = get_new_field(field.nrows, field.ncols);

    // Look att all positions in old field
    for row in 0..field.nrows {
        for col in 0..field.ncols {
            // Count neighbours at this postion
            let count = count_neighbours(&field, row, col);
            let current_cell = field.data[get_index(&field, row, col)];
            new_field.data[get_index(&field, row, col)] = match (current_cell, count) {
                (1, 2) | (1, 3) | (0, 3) => 1,
                _ => 0,
            };
        }
    }
    return new_field;
}

fn main() {
    let window = initscr();

    let ncols = (getmaxx(window) - 1) as i16;
    let nrows = (getmaxy(window) - 1) as i16;

    let mut field = get_new_random_field(nrows, ncols);
    let mut new_field;

    let stop = 1000;

    for index in 1..stop {
        new_field = update_field(&field);
        print_field_diff(&new_field, &field);
        field = new_field;
        if index < stop - 1 {
            clear();
        }
    }

    printw("Press key\n");
    getch();
    endwin();
}
