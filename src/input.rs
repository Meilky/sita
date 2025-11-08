use std::{env, path::Path};

pub enum ColorType {
    MONOCHROME,
    COLOR,
}

pub struct Args {
    pub color_type: ColorType,
    pub scale: u8,
    pub input_file_path: String,
    pub output_file_path: String,
}

impl Args {
    pub fn from_args() -> Args {
        let args: Vec<String> = env::args().collect();

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

        let mut scale: u8 = 1;

        if args.len() >= 4 {
            scale = args[3].parse().unwrap();
        }

        let mut color_type = ColorType::MONOCHROME;

        if args.len() >= 5 {
            color_type = match args[4].as_str() {
                "color" => ColorType::COLOR,
                "monochrome" => ColorType::MONOCHROME,
                _ => ColorType::MONOCHROME,
            };
        }
        Args {
            color_type,
            scale,
            input_file_path,
            output_file_path,
        }
    }
}
