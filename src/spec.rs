use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
};

use crate::test::Test;

#[derive(Debug, Deserialize, Serialize)]
pub struct Spec {
    pub name: String,
    pub tests: HashMap<String, Test>,
}

impl Spec {
    pub fn build(&self) -> Result<(), Box<dyn Error>> {
        println!("# {}\n", self.name);

        for (_test_id, test) in self.tests.iter() {
            println!("## {}\n", test.name);
            test.build()?;
        }

        Ok(())
    }
}
