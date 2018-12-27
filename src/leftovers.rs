fn count_live_cells(field: Field) -> u16 {
    let mut count: u16 = 0;
    for row in 0..field.nrows {
        for col in 0..field.ncols {
            // Count neighbours at this postion
            count += (field.data[get_index(&field, row, col)] as u16);
        }
    }
    count
}

fn get_new_field_with_wanderer() -> Field {
    let mut field = get_new_field();
    field.data[get_index(&field, 21, 21)] = 1;
    field.data[get_index(&field, 22, 20)] = 1;
    field.data[get_index(&field, 23, 20)] = 1;
    field.data[get_index(&field, 23, 21)] = 1;
    field.data[get_index(&field, 23, 22)] = 1;
    field
}
