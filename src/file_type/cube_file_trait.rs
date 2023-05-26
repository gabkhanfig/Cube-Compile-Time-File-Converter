use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use image::GenericImageView;
use colored::Colorize;

pub fn load_file_to_string(file_path: &String) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file").to_string()
}

pub fn get_file_name_no_folder(file_path: &String) -> String {
    let pos = file_path.rfind('/');
    let mut start_pos: usize = 0;
    if pos.is_some() {
        start_pos = pos.unwrap() + 1;
    }
    let end_pos = file_path.rfind('.').expect("file path must have period for extention");

    return file_path.clone()[start_pos..end_pos].to_string();
}

pub fn get_extension_from_filename(filename: &str) -> &str {
    let ext = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str);
    assert!(ext.is_some(), "Unable to get extension for file \"{filename}\"");
    return ext.unwrap();
}

pub fn create_variable_name(file_path: &str) -> String {
    let pos = file_path.rfind('/');
    let mut start_pos: usize = 0;
    if pos.is_some() {
        start_pos = pos.unwrap() + 1;
    }
    let end_pos = file_path.rfind('.').expect("file path must have period for extention");

    let substr_file_name = file_path.clone()[start_pos..end_pos].to_string();

    return format!("{}{}{}{}", "generated_", substr_file_name.as_str(),'_', get_extension_from_filename(file_path));
}

fn make_cpp_contents_from_file_contents(file_contents: String) -> String {
    let mut compile_string = String::from("R\"(");
    for c in file_contents.chars() {
        match c {
            '"' => compile_string.push_str("\""),
            '\\' => compile_string.push_str("\\"),
            _ => compile_string.push(c),
        };
    }
    compile_string.push_str(")\";");
    return compile_string;
}

fn file_exists(file_path: &str) -> bool {
    return std::path::Path::new(file_path).exists();
}

pub trait CubeFile {
    fn cpp_string(&self) -> String;

    fn header_string(&self) -> String;
    
    fn parse_success_message(&self);
}

pub struct TextFile {
    pub file_path: String,
    pub file_name: String,
    pub contents: String,
    pub variable_name: String,
}

impl TextFile {
    pub fn new(file_path: &String) -> Box<dyn CubeFile> {
        Box::new(TextFile {
            file_path: file_path.clone(),
            file_name: get_file_name_no_folder(file_path),
            contents: make_cpp_contents_from_file_contents(load_file_to_string(file_path)),
            variable_name: create_variable_name(file_path)
        })
    }
}

impl CubeFile for TextFile {
    fn cpp_string(&self) -> String {
        format!("{}{}{}{}",
            "const char* ",
            self.variable_name,
            " = ",
            self.contents
        )
    }

    fn header_string(&self) -> String {
        format!("{}{}{}{}{}",
            "// Generated text file from ",
            self.file_path,
            "\nextern const char* ",
            self.variable_name,
            ";"
        )
    }

    fn parse_success_message(&self) {
        println!("{}{}{}{}", "[Cube Compile Time File Parser]".green(), ' ', self.file_path.cyan(), " Successfully parsed text file");
    }
}

pub struct ImageFile {
    pub file_path: String,
    pub file_name: String,
    pub variable_name: String,
    pub contents: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub total_bytes: usize,
}

impl ImageFile {
    pub fn new(file_path: &String) -> Box<dyn CubeFile> {
        let img = image::open(file_path).unwrap();
        let image_bytes: Vec<u8> = img.as_bytes().to_vec();
        let (image_width, image_height) = img.dimensions();
        let image_total_bytes = image_bytes.len();
        Box::new(ImageFile { 
            file_path: file_path.clone(), 
            file_name: get_file_name_no_folder(file_path), 
            variable_name: create_variable_name(file_path), 
            contents: image_bytes, 
            width: image_height, 
            height: image_width, 
            total_bytes: image_total_bytes 
        })
    }
}

impl CubeFile for ImageFile {
    fn cpp_string(&self) -> String {
        format!("{}{}{}{:?}{}{}{}{}{}{}{}{}{}{}{}{}",
            "const unsigned char* ",
            self.variable_name,
            " = ",
            self.contents,
            ";\nconst unsigned int ",
            self.variable_name,
            "_width = ",
            self.width,
            "\nconst unsigned int ",
            self.variable_name,
            "_height = ",
            self.height,
            "\nconst unsigned int ",
            self.variable_name,
            "_total_bytes = ",
            self.total_bytes,
        )
    }

    fn header_string(&self) -> String {
        format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "// Generated image file from ",
            self.file_path,
            "\nextern const unsigned char* ",
            self.variable_name,
            ";\n// Generated image pixel width from ",
            self.file_path,
            "\nextern const unsigned int ",
            self.variable_name,
            "_width;\n// Generated image pixel height from ",
            self.file_path,
            "\nextern const unsigned int ",
            self.variable_name,
            "_height;\n// Generated total image bytes from ",
            self.file_path,
            "\nextern const unsigned int ",
            self.variable_name,
            "_total_bytes;",
        )
    }

    fn parse_success_message(&self) {
        println!("{}{}{}{}", "[Cube Compile Time File Parser]".green(), ' ', self.file_path.cyan(), " Successfully parsed image file");
    }
}
