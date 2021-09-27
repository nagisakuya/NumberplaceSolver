use std::{
    fmt::{self},
};

#[derive(Clone, Copy)]
pub struct Grid {
    placed_number: Option<u8>,
    placeble_numbers: [bool; 9],
}
impl Grid {
    pub fn new() -> Self {
        Grid {
            placed_number: None,
            placeble_numbers: [false; 9],
        }
    }
    pub fn set(&mut self, num: u8) -> Result<&mut Grid, &'static str> {
        match num {
            0 => self.placed_number = None,
            1..=9 => {
                self.placed_number = Some(num);
                self.reset_placeble(false);
                self.set_placeble(num as usize, true)?;
            }
            _ => return Err("1から9までの数字を入れろ--set"),
        }
        Ok(self)
    }
    pub fn get(&self) -> Option<u8> {
        self.placed_number
    }
    pub fn reset_placeble(&mut self, set: bool) -> () {
        self.placeble_numbers = [set; 9];
    }
    pub fn set_placeble(&mut self, num: usize, bool: bool) -> Result<&mut Grid, &'static str> {
        match num {
            1..=9 => self.placeble_numbers[num - 1] = bool,
            _ => return Err("1から9までの数字を入れろ--set_placeble"),
        }
        Ok(self)
    }
    pub fn get_placeble(&self, num: usize) -> bool {
        self.placeble_numbers[num - 1]
    }
    pub fn count_placeble(&self) -> u8 {
        let mut counter = 0;
        for i in self.placeble_numbers.iter() {
            if *i {
                counter += 1
            }
        }
        counter
    }
}

pub struct NumberPlace {
    field: [[Grid; 9]; 9],
}

impl NumberPlace {
    pub fn new() -> Self {
        NumberPlace {
            field: [[Grid::new(); 9]; 9],
        }
    }
    pub fn set_field(&mut self, from: [[u8; 9]; 9]) -> Result<&mut NumberPlace, &'static str> {
        for i in 0..9 {
            for j in 0..9 {
                self.field[i][j].set(from[i][j])?;
            }
        }
        Ok(self)
    }
    #[allow(dead_code)]
    pub fn set_number(
        &mut self,
        position: (usize, usize),
        num: u8,
    ) -> Result<&mut NumberPlace, &'static str> {
        self.field[position.1][position.0].set(num)?;
        Ok(self)
    }
    pub fn get_number(&self, x: usize, y: usize) -> Option<u8> {
        self.field[y][x].placed_number
    }
    pub fn get_grid(&self, x: usize, y: usize) -> &Grid {
        &self.field[y][x]
    }
    pub fn get_mut_grid(&mut self, x: usize, y: usize) -> &mut Grid {
        &mut self.field[y][x]
    }
    pub fn check_overlap<F>(get: F) -> bool
    where
        F: Fn(usize, usize) -> Option<u8>,
    {
        for i in 0..9 {
            let mut array = [false; 10];
            for j in 0..9 {
                match get(i, j) {
                    Some(o) => {
                        if array[o as usize] {
                            return true;
                        } else {
                            array[o as usize] = true;
                        }
                    }
                    _ => (),
                }
            }
        }
        false
    }
    pub fn consistency_check(&self) -> bool {
        if NumberPlace::check_overlap(|x, y| self.get_number(x, y)) {
            return false;
        }
        if NumberPlace::check_overlap(|y, x| self.get_number(x, y)) {
            return false;
        }
        if NumberPlace::check_overlap(|i, j| {
            self.get_number((i / 3) * 3 + j / 3, (i % 3) * 3 + j % 3)
        }) {
            return false;
        }
        true
    }
    pub fn for_each_grid<F>(&mut self, mut func: F) -> Result<(), &'static str>
    where
        F: FnMut(&mut Grid, usize, usize) -> Result<(), &'static str>,
    {
        for (y, line) in self.field.iter_mut().enumerate() {
            for (x, item) in line.iter_mut().enumerate() {
                func(item, x, y)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for NumberPlace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, &line) in self.field.iter().enumerate() {
            for (x, &item) in line.iter().enumerate() {
                match item.placed_number {
                    Some(v) => write!(f, "{}", v)?,
                    None => write!(f, " ")?,
                };
                if x == 2 || x == 5 {
                    write!(f, "|")?;
                }
            }
            write!(f, "\n")?;
            if y == 2 || y == 5 {
                write!(f, "-----------\n")?;
            }
        }
        Ok(())
    }
}
