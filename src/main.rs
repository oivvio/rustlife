extern crate ncurses;

use ncurses::*;

use std::{thread, time};

// extern crate rand;

use rand::Rng;

const NCOLS: i16 = 100;
const NROWS: i16 = 40;
const NCELLS: i16 = NCOLS * NROWS + 1000;

type Field = ([u8; NCELLS as usize], i16, i16);

fn print_field(field: Field) {
    printw("\n");

    for row in 0..field.1 {
        for col in 0..field.2 {
            let index = (col + (row * field.2)) as usize;
            let val = field.0[index];
            match val {
                0 => printw(" "),
                _ => printw("X"),
            };
        }
        printw("\n");
    }
    refresh();
}

fn get_new_field() -> Field {
    let field = ([0; NCELLS as usize], NROWS, NCOLS);
    return field;
}

fn get_new_random_field() -> Field {
    let mut field = ([0; NCELLS as usize], NROWS, NCOLS);

    for row in 0..NROWS {
        for col in 0..NCOLS {
            let index = get_index(row, col);
            field.0[index] = rand::thread_rng().gen_range(0, 2);
        }
    }

    return field;
}

fn get_index(row: i16, col: i16) -> usize {
    let mut row = row;
    let mut col = col;

    if row < 0 {
        row = NROWS;
    }
    if row > NROWS {
        row = 0;
    }

    if col < 0 {
        col = NCOLS;
    }

    if col > NCOLS {
        col = 0;
    }

    (col + (row * NCOLS)) as usize
}

fn count_neighbours(field: Field, row: i16, col: i16) -> u8 {
    let mut count: u8 = 0;

    count += field.0[get_index(row - 1, col - 1)];
    count += field.0[get_index(row - 1, col)];
    count += field.0[get_index(row - 1, col + 1)];

    count += field.0[get_index(row, col - 1)];
    count += field.0[get_index(row, col + 1)];

    count += field.0[get_index(row + 1, col - 1)];
    count += field.0[get_index(row + 1, col)];
    count += field.0[get_index(row + 1, col + 1)];

    count
}

fn count_live_cells(field: Field) -> u16 {
    let mut count: u16 = 0;
    for row in 0..NROWS {
        for col in 0..NCOLS {
            // Count neighbours at this postion
            count += (field.0[get_index(row, col)] as u16);
        }
    }

    count
}

fn update_field(field: Field) -> Field {
    // Init new empty field
    let mut new_field = get_new_field();

    // Look att all positions in old field
    for row in 0..NROWS {
        for col in 0..NCOLS {
            // Count neighbours at this postion
            let count = count_neighbours(field, row, col);
            new_field.0[get_index(row, col)] = match (field.0[get_index(row, col)], count) {
                (1, 2) | (1, 3) => 1,
                (0, 3) => 1,
                _ => 0,
            };
        }
    }
    return new_field;
}

fn main() {
    let ten_millis = time::Duration::from_millis(50);

    initscr();
    let mut field = get_new_random_field();
    let stop = 1000;

    for index in 1..stop {
        field = update_field(field);
        print_field(field);

        if index < stop - 1 {
            clear();
        }

    }
    printw("Press key\n");
    getch();

    endwin();
}
