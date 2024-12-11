use std::fs;
use std::io;

use crate::core::characteristics::Characteristic;

pub struct Styles;

impl Characteristic for Styles {
    fn get_header(&self) -> String {
        "This is the style you use to talk in".to_string()
    }

    fn get_traits(&self, character_name: &str) -> io::Result<String> {
        let path = format!("./characters/{}/styles_traits.txt", character_name);
        fs::read_to_string(&path)
    }
}