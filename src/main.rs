extern crate ncurses;

use ncurses::*;

use std::{thread, time};

// extern crate rand;

use rand::Rng;

const NCOLS: u16 = 100;
const NROWS: u16 = 40;
const NCELLS: u16 = NCOLS * NROWS;

type Field = ([u8; NCELLS as usize], u16, u16);

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
        printw("|\n");
    }
    refresh();
}

fn get_new_field() -> Field {
    let field = ([0; NCELLS as usize], NROWS, NCOLS);
    return field;
}

fn get_new_random_field() -> Field {
    let mut field = ([0; NCELLS as usize], NROWS, NCOLS);

    for row in 0..field.1 {
        for col in 0..field.2 {
            let index = (col + (row * field.2)) as usize;
            field.0[index] = rand::thread_rng().gen_range(0, 2);
        }
    }

    return field;
}

fn get_index(row: u16, col: u16) -> usize {

    // if row is out of bounds wrap around
    // if row
    // if col is out of bounds wrap around

}

fn count_neighbours(field: Field, row: u16, col: u16) -> u8 {
    let count: u8 = 0;

    //field.0[
    return count;
}

fn update_field(field: Field) -> Field {
    // Init new empty field
    let mut new_field = get_new_field();

    // Look att all positions in old field
    for row in 0..field.1 {
        for col in 0..field.2 {
            // Count neighbours at this postion
            let count = count_neighbours(field, row, col);
        }
    }

    return new_field;
}

fn main() {
    let ten_millis = time::Duration::from_millis(50);

    initscr();
    /* Print to the back buffer. */
    // let mut field = ([0; 100], 10, 10);
    let mut field = get_new_random_field();
    for _ in 1..100 {
        let field = update_field(field);
        print_field(field);
        printw("Press key\n");
        clear();
    }

    endwin();
}
