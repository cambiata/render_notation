#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::useless_format)]

pub mod prelude;
pub mod render;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::fonts::Merriweather_Regular::get_path;

    #[test]
    fn example() {
        let path = get_path('a');
        dbg!(path);
    }
}
