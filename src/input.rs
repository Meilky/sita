use std::{env, path::Path};

pub struct Args {
    pub scale: u8,
    pub input_file_path: String,
    pub output_file_path: String,
}

impl Args {
    pub fn from_args() -> Args {
        let args: Vec<String> = env::args().collect();

        let scale: u8 = 10;

        if args.len() == 1 {
            panic!("You need to provide a path to the input file!");
        }

        if args.len() == 2 {
            panic!("You need to provide a path to the output file!");
        }

        let input_file_path = args[1].clone();
        let output_file_path = args[2].clone();

        if Path::new(&input_file_path).exists() == false {
            panic!("The input file must exists!");
        }

        Args {
            scale,
            input_file_path,
            output_file_path,
        }
    }
}
