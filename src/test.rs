use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    io::Read,
    process::{
        Command,
        Stdio,
    },
    str,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Test {
    pub name: String,
    pub command: String,
    pub output: String,
}

impl Test {
    pub fn build(&self) -> Result<(), Box<dyn Error>> {
        let mut output = Vec::new();
        {
            let mut child = Command::new("sh")
                .arg("-c")
                .arg(&self.command)
                .stdout(Stdio::piped())
                .spawn()?;

            //TODO: timeout, async read
            let mut stdout = child.stdout.take().unwrap();
            loop {
                let mut buf = [0; 4096];
                let count = stdout.read(&mut buf)?;
                output.extend_from_slice(&buf[..count]);

                if let Some(status) = child.try_wait()? {
                    println!("    * status: {}", status);
                    if ! status.success() {
                        println!("    * exit status indicates failure");
                    }
                    break;
                }
            }
        }

        let output = str::from_utf8(&output)?;
        println!("    * output:\n```\n{}\n```", output);
        if output != self.output {
            println!("    * expected:\n```\n{}\n```", self.output);
            println!("    * output does not match expected");
        }

        Ok(())
    }
}
