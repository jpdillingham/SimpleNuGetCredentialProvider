use std::env;
use std::io::Result;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<()> {
    // collect the command line arguments; NuGet passes the URL it is seeking credentials
    // for, along with some miscellaneous options that we'll most likely ignore.
    let args: Vec<String> = env::args().collect();

    // dump these args to a file so we can see what they are
    let mut file = File::create("args.txt")?;

    for x in &args {
        let y = format!("{}{}", x, "\n");
        let z = y.as_bytes();
        file.write_all(z)?;
    }

    // print the args to the screen, too
    for x in &args {
        println!("{x}");
    }

    Ok(())
}