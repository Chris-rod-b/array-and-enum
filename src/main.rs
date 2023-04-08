use chrono::prelude::*;
mod custom_array_print;

enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black: u8}
}

fn main() {
    array();
    array_of_array();
    custom_array_print::print();

    let now = Local::now();
    let day_of_week = match now.weekday() {
        Weekday::Sun => DayOfWeek::Sunday,
        Weekday::Mon => DayOfWeek::Monday,
        Weekday::Tue => DayOfWeek::Tuesday,
        Weekday::Wed => DayOfWeek::Wednesday,
        Weekday::Thu => DayOfWeek::Thursday,
        Weekday::Fri => DayOfWeek::Friday,
        Weekday::Sat => DayOfWeek::Saturday,
    };

    println!("Is it weekend today? {}", weekend(day_of_week));
    color();
}

fn array() {
    let grade:[f32; 4] = [10.0, 8.0, 9.5, 6.0];

    //reply the same number in an array
    /* let grade:[f32; 4] = [6.5; 4]; */

    for i in 0..grade.len() {
        println!(
            "Grade {} = {}",
            i + 1, grade[i],
        );
    }
}

fn array_of_array() {
    let two_d_array:[[f32; 3]; 2] = [
        [0.0, 1.2, 0.1], 
        [1.3, 0.3, 1.4]
    ];

    for line in two_d_array {
        for item in line {
            println!("Item = {}", item);
        }
    }

    println!("{:?}", two_d_array);
}

fn weekend(day_of_week: DayOfWeek) -> bool {
    match day_of_week {
        DayOfWeek::Sunday | DayOfWeek::Saturday => true,
        _ => false
    }
}

fn color() {
    let color = Color::CymkColor { cyan: 30, magenta: 45, yellow: 21, black: 255 };

    println!("Color = {}", match color {
       Color::Red => "Red!",
       Color::Blue => "Blue!",
       Color::Green => "Green!",
       Color::RgbColor(0, 0, 0) | 
       Color::CymkColor { cyan: _, magenta: _, yellow: _, black: 255 } => "Black!",
       Color::RgbColor(_, _, _) => "Unknown RGB!",
       Color::CymkColor {cyan: _, magenta: _, yellow: _, black: _ } => "Unknown CYMK!"
    });
}