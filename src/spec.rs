use serde::{Deserialize, Serialize};
use std::{
    error::Error,
};

use crate::test::Test;

#[derive(Debug, Deserialize, Serialize)]
pub struct Spec {
    pub name: String,
    #[serde(with = "tuple_vec_map")]
    pub tests: Vec<(String, Test)>,
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
