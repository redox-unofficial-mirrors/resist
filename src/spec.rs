use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    collections::HashMap,
};

use crate::test::Test;

#[derive(Debug, Deserialize, Serialize)]
pub struct Spec {
    pub name: String,
    pub tests: HashMap<String, Test>,
}

impl Spec {
    pub fn build(&self) -> Result<(), Box<dyn Error>> {
        println!("* spec '{}'", self.name);
        for (test_id, test) in self.tests.iter() {
            println!("  * test {}: '{}'", test_id, test.name);
            test.build()?;
        }
        Ok(())
    }
}
