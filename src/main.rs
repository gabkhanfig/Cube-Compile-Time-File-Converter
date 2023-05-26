use std::env;
use std::fs::File;
use std::io::Write;
use colored::Colorize;

mod file_type;
use crate::file_type::cube_file_trait::CubeFile;
use crate::file_type::cube_file_trait::TextFile;
use crate::file_type::cube_file_trait::ImageFile;
use crate::file_type::cube_file_trait::load_file_to_string;
use crate::file_type::cube_file_trait::get_extension_from_filename;

struct Arguments
{
    txt_path_in: String,
    header_path_out: String
}

enum FileType {
    Text,
    Image
}

impl FileType {
    fn from(extention: &str) -> FileType {
        if extention.eq("png") {
            FileType::Image
        }
        else {
            FileType::Text
        }
    }
}

fn main() {
    let cli_args_in: Vec<String> = env::args().collect();
    assert!(cli_args_in.len() == 3, "2 command line arguments are required.\n1: File path to get all files to parse\n2: Name of the resulting c++ header file\n Example Of Correct Usage: run.exe files.txt header.h\n");
    let args = Arguments{
        txt_path_in: cli_args_in[1].clone(),
        header_path_out: cli_args_in[2].clone()
    };
    let files = get_all_file_names(&args.txt_path_in);

    let mut generated_files: Vec<Box<dyn CubeFile>> = Vec::new();
  
    let mut success = 0;
    let mut fail = 0;

    for file in files {
        if !std::path::Path::new(file.as_str()).exists() {
            println!("{}{}", "File doesn't exist: ", file);
            continue;
        }
        let generated_file = match FileType::from(get_extension_from_filename(&file)) {
            FileType::Text => TextFile::new(&file),
            FileType::Image => ImageFile::new(&file),
        };
        generated_file.parse_success_message();
        success += 1;
        generated_files.push(generated_file);
    }


    let mut header_string = String::from("#pragma once\n\n");
    let mut cpp_string = String::from("");
    for file in generated_files {
        header_string.push_str(file.header_string().as_str());
        header_string.push_str("\n\n");

        cpp_string.push_str(file.cpp_string().as_str());
        cpp_string.push_str("\n\n");
    }

    let mut header_file = File::create(format!("{}{}", args.header_path_out, ".h")).expect("Unable to create header file!");
    let mut cpp_file = File::create(format!("{}{}", args.header_path_out, ".cpp")).expect("Unable to create cpp file!");
    header_file.write_all(header_string.as_bytes()).expect("Unable to write to header file!");
    cpp_file.write_all(cpp_string.as_bytes()).expect("Unable to write to cpp file!");

    println!("{}", "\nFinished running Cube Compile Time File Converter".bold());
    let success_num_string = format!("{success} files converted").green().bold();
    let fail_num_string = format!("{fail} files not converted").magenta().bold();
    println!("{success_num_string}");
    println!("{fail_num_string}");

}

// Get all of the files to parse
fn get_all_file_names(txt_containing_files: &String) -> Vec<String> {
    let contents = load_file_to_string(txt_containing_files);
    let mut file_names: Vec<String> = contents.split("\r\n").map(|s| s.to_string()).collect();
    if file_names[file_names.len() - 1].len() == 0 { 
        file_names.remove(file_names.len() - 1);
    }
    return file_names;
}