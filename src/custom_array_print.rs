use std::fmt;

struct MyArray([[u32; 3]; 3]);

impl fmt::Display for MyArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for elem in row {
                write!(f, "{:<4}", elem)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn print() {
    let my_array = MyArray([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8 ,9]
    ]);

    println!("{}", my_array);
}