use crate::numberplace::NumberPlace;

pub struct NumberPlaceSolver {
    pub field: NumberPlace,
}

impl NumberPlaceSolver {
    fn reset_placeble(&mut self) -> Result<(), &'static str> {
        self.field.for_each_grid(|item, _x, _y| {
            match item.get() {
                Some(num) => {
                    item.reset_placeble(false);
                    item.set_placeble(num as usize, true)?;
                }
                None => item.reset_placeble(true),
            }
            Ok(())
        })?;
        self.check_placeble()
    }
    fn check_placeble(&mut self) -> Result<(), &'static str> {
        for x in 0..9 {
            for y in 0..9 {
                match self.field.get_grid(x, y).get() {
                    Some(num) => self.place((x, y), num)?,
                    _ => (),
                }
            }
        }
        Ok(())
    }
    fn check_placeble_guess(&mut self) -> Result<bool, &'static str> {
        for x in 0..3 {
            for y in 0..3 {
                for num in 1..=9 {
                    let mut row = [false; 3];
                    let mut column = [false; 3];
                    for i in 0..9 {
                        if self
                            .field
                            .get_grid(x * 3 + i % 3, y * 3 + i / 3)
                            .get_placeble(num)
                        {
                            row[i / 3] = true;
                            column[i % 3] = true;
                        }
                    }

                    let mut func = |array: [bool; 3],
                                    check: &(dyn Fn(usize) -> bool),
                                    func: &(dyn Fn(usize, usize) -> (usize, usize))|
                     -> Result<bool, &'static str> {
                        let mut counter = 0;
                        let mut index = 0;
                        for (i, item) in array.iter().enumerate() {
                            if *item {
                                counter += 1;
                                index = i;
                            }
                        }
                        if counter == 1 {
                            for i in 0..9 {
                                if check(i) {
                                    let temp = func(i, index);
                                    let grid = self.field.get_mut_grid(temp.0, temp.1); {
                                        if grid.get_placeble(num){
                                            grid.set_placeble(num as usize, false)?;
                                            return Ok(true);
                                        }
                                    }
                                }
                            }
                        }
                        Ok(false)
                    };
                    if func(row, &|i| !(x * 3 <= i && i <= x * 3 + 3), &|i, index| {
                        ( i,y * 3 + index)
                    })? {
                        return Ok(true);
                    };
                    if func(column, &|i| !(y * 3 <= i && i <= y * 3 + 3), &|i, index| {
                        ( x* 3 + index,i)
                    })? {
                        return Ok(true);
                    };
                }
            }
        }
        Ok(false)
    }
    fn place(&mut self, position: (usize, usize), num: u8) -> Result<(), &'static str> {
        self.field
            .get_mut_grid(position.0, position.1)
            .set(num as u8)?;

        //remove placebles
       
                let mut func = |tx, ty, num: u8| -> Result<(), &'static str> {
                    if position.0 == tx && position.1 == ty {
                        return Ok(());
                    }
                    self.field
                        .get_mut_grid(tx, ty)
                        .set_placeble(num as usize, false)?;
                    Ok(())
                };
                for i in 0..9 {
                    func(position.0, i, num)?;
                    func(i, position.1, num)?;
                    func(
                        (position.0 / 3) * 3 + i / 3,
                        (position.1 / 3) * 3 + i % 3,
                        num,
                    )?;
                }
            
        Ok(())
    }
    fn check_singleton(&mut self) -> Result<bool, &'static str> {
        for x in 0..9 {
            for y in 0..9 {
                match self.field.get_grid(x, y).count_placeble() {
                    0 => {
                        println!("何も入り得ないマスがあるよ！")
                    }
                    1 => {
                        if self.field.get_grid(x, y).get().is_some() {
                            continue;
                        }
                        for num in 1..=9 {
                            if self.field.get_grid(x, y).get_placeble(num) {
                                self.place((x, y), num as u8)?;
                            }
                        }
                        return Ok(true);
                    }
                    _ => (),
                }
            }
        }
        let mut func =
            |pos: &(dyn Fn(usize, usize) -> (usize, usize))| -> Result<bool, &'static str> {
                for num in 1..=9 {
                    for i in 0..9 {
                        let mut counter = 0;
                        let mut index = (0, 0);
                        for j in 0..9 {
                            let temp = pos(i, j);
                            if self.field.get_grid(temp.0, temp.1).get_placeble(num) {
                                counter += 1;
                                index = temp;
                            }
                        }
                        if counter == 1 {
                            if self.field.get_mut_grid(index.0, index.1).get().is_some() {
                                continue;
                            }
                            self.field.get_mut_grid(index.0, index.1).set(num as u8)?;
                            return Ok(true);
                        }
                    }
                }
                Ok(false)
            };
        if func(&|i, j| (i, j))? {
            return Ok(true);
        };
        if func(&|i, j| (j, i))? {
            return Ok(true);
        };
        if func(&|i, j| ((i / 3) * 3, j))? {
            return Ok(true);
        };

        Ok(false)
    }
    pub fn solve(&mut self) -> Result<(), &'static str> {
        println!("{}", self.field);
        self.reset_placeble()?;
        loop {
            if self.check_singleton()? {continue;};
            if self.check_placeble_guess()? {continue;};
            break;
        }
        println!("{}", self.field);
        Ok(())
    }
}
