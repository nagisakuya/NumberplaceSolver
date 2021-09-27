mod numberplace;
mod numberplace_solver;
mod test;
use std::fs;

use crate::numberplace::NumberPlace;

fn parse(mut data: String) -> [[u8; 9]; 9] {
    data += ",";
    let mut re = [[0u8; 9]; 9];
    let mut chars = data.chars();
    let mut i = 0;
    while i < 81 {
        match chars.next().unwrap() {
            ',' | '\r' => re[i / 9][i % 9] = 0,
            '\n' => continue,
            x => {
                re[i / 9][i % 9] = (x as i32 - '0' as i32) as u8;
                chars.next();
            }
        }
        i += 1;
    }
    re
}

fn main() {
    let mut test = numberplace_solver::NumberPlaceSolver {
        field: NumberPlace::new(),
    };
    test.field
        .set_field(parse(
            fs::read_to_string("./source/test_medium.csv").unwrap(),
        ))
        .expect("");
    test.solve().expect("msg");
    print!("{}", test.field.consistency_check());
}
