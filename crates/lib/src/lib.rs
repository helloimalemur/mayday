mod api_keys;
pub use api_keys::*;
mod entities;
pub use entities::*;
use std::fs;
use std::io::BufRead;

pub fn load_keys_from_file() -> Vec<String> {
    let mut keys: Vec<String> = vec![];
    let file = fs::read("config/api_keys").unwrap();
    for line in file.lines() {
        keys.push(line.unwrap())
    }
    // println!("{:#?}", keys);
    keys
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
