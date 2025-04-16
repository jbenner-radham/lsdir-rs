use itertools::Itertools;
use std::env;
use std::io::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let path_name = if args.len() > 1 { &args[1] } else { "." };

    Path::new(path_name)
        .read_dir()?
        .filter(|dir_entry| dir_entry.as_ref().unwrap().file_type().unwrap().is_dir())
        .map(|dir_entry| dir_entry.unwrap().file_name())
        .sorted()
        .for_each(|dir| println!("{}", dir.to_str().unwrap()));

    Ok(())
}
