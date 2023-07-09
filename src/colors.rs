/// 
/// Copyright 2023, [object Object]
/// Licensed under MIT
///
/// Warnings:
/// Please check every file before deleting it, i am not responsible for any damage caused by this tool!
/// 

macro_rules! color_fn {
    ($name:ident, $code:expr) => {
        fn $name(&self) -> String {
            return format!("\x1b[{}m{}\x1b[0m", $code, self);
        }
    };

    ($name:ident, $code:expr, $bg_code:expr) => {
        fn $name(&self) -> String {
            return format!("\x1b[{};{}m{}\x1b[0m", $code, $bg_code, self);
        }
    };
}

pub trait Colors: std::fmt::Display {
    color_fn!(red, 31);
    color_fn!(green, 32);
    color_fn!(blue, 34);
    color_fn!(yellow, 33);
    color_fn!(magenta, 35);
    color_fn!(cyan, 36);
    color_fn!(white, 37);
    color_fn!(black, 30);

    color_fn!(bright_red, 91);
    color_fn!(bright_green, 92);
    color_fn!(bright_blue, 94);
    color_fn!(bright_yellow, 93);
    color_fn!(bright_magenta, 95);
    color_fn!(bright_cyan, 96);
    color_fn!(bright_white, 97);

    color_fn!(bold_red, 31, 1);
    color_fn!(bold_green, 32, 1);
    color_fn!(bold_blue, 34, 1);
    color_fn!(bold_yellow, 33, 1);
    color_fn!(bold_magenta, 35, 1);
    color_fn!(bold_cyan, 36, 1);
    color_fn!(bold_white, 37, 1);
    color_fn!(bold_black, 30, 1);

    color_fn!(bold_bright_red, 91, 1);
    color_fn!(bold_bright_green, 92, 1);
    color_fn!(bold_bright_blue, 94, 1);
    color_fn!(bold_bright_yellow, 93, 1);
    color_fn!(bold_bright_magenta, 95, 1);
    color_fn!(bold_bright_cyan, 96, 1);
    color_fn!(bold_bright_white, 97, 1);

    color_fn!(bg_red, 31, 41);
    color_fn!(bg_green, 32, 42);
    color_fn!(bg_blue, 34, 44);
    color_fn!(bg_yellow, 33, 43);
    color_fn!(bg_magenta, 35, 45);
    color_fn!(bg_cyan, 36, 46);
    color_fn!(bg_white, 37, 47);
    color_fn!(bg_black, 30, 40);

    color_fn!(bg_bright_red, 91, 101);
    color_fn!(bg_bright_green, 92, 102);
    color_fn!(bg_bright_blue, 94, 104);
    color_fn!(bg_bright_yellow, 93, 103);
    color_fn!(bg_bright_magenta, 95, 105);
    color_fn!(bg_bright_cyan, 96, 106);
    color_fn!(bg_bright_white, 97, 107);


}

impl Colors for &str {}