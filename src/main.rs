use std::error::Error;
use std::fs;
use std::time::SystemTime;
use std::cmp::Reverse;
use std::env;
use walkdir::WalkDir;

/// One arg <folder_path> which provides the path to the files that need to be renamed is required.
/// The other arg <list_of_filetyps> is optional. If used it will provide an alternate list of filetypes to use.
///
/// If too many or not enough args are inputted the program will exit with -1. 
fn main() {
    let args_length = env::args().len();
    if args_length < 2 || args_length > 3 {
        println!("Need at least one arg! required arg: <folder_path> optional arg: <list_of_filetypes>");
        std::process::exit(-1);
    } else if args_length == 2 {
        let folder_path = env::args().nth(1).unwrap();
        let result = arg_parser_2(folder_path);
        println!("{:?}", result);
    } else if args_length == 3 {
        let folder_path = env::args().nth(1).unwrap();
        let filetypes_path = env::args().nth(2).unwrap();
        let result = arg_parser_3(folder_path, filetypes_path);
        println!("{:?}", result);
    }
    std::process::exit(0);
}

/// Parses the arg for the path to the directory containing the files to be renamed. 
/// Uses a bundled list of filetypes to provide the filetypes used to idenitfy pictures.
/// # Filetypes in List
/// .jpg .jpeg .png .mp4 .dng .gif .nef .bmp .jpe .jif .jfif .jfi
/// .webp .tiff .tif .psd .raw .arw .cr2 .nrw .k25 .dib .heif .heic .ind .indd .indt .jp2 .j2k .jpf
/// .jpx .jpm .mj2 .svg .svgz .ai .eps .pdf .xcf .cdr .sr2 .orf .bin .afphoto .mkv
fn arg_parser_2(folder_path: String) -> Result<bool, Box<dyn Error>> {
    let filetypes = include_str!("_list_of_filetypes.txt").to_string();
    let alt_filetypes = alt_get_filetypes(filetypes)?;
    directory_walker(&folder_path, alt_filetypes)?;
    return Ok(true)
}

/// Parses two inputted args where the first one is the path to the directory with the files to be renamed 
/// and the second one is the path to a list containing filetypes. This supports additional file formats.
///
/// "// and "# can be used as comments in the file. The file is read in as a String.
fn arg_parser_3(folder_path: String, filetypes_path: String) -> Result<bool, Box<dyn Error>> {
    let filetypes = get_filetypes(&filetypes_path)?;
    directory_walker(&folder_path, filetypes)?;
    return Ok(true)
}

/// Walks the directories to ensure that all pictures get renamed. If there are pictures in subdirectories they will get renamed.
/// # Behavior
/// If a path to a directory that does not exist is provided the function will not return an error. If a directory doesn't exist 
/// it means that there are no files to be renamed.
fn directory_walker(folder_path: &str, filetypes: Vec<String>) -> Result<bool, Box<dyn Error>> {
    println!("Preparing to rename files in {}", folder_path);
    let mut directories: Vec<walkdir::DirEntry> = WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()).collect();
    directories.retain(|entry| fs::metadata(entry.path()).unwrap().is_dir());
    for directory in directories {
        file_namer(directory.path(), &filetypes)?;
    }
    return Ok(true)
}

/// This renames the files with the specified filetypes.
fn file_namer(folder_path: &std::path::Path, filetypes: &Vec<String>) -> Result<bool, Box<dyn Error>> {
    std::env::set_current_dir(folder_path)?;
    let sys_time = SystemTime::now();
    let mut paths: Vec<fs::DirEntry> = fs::read_dir(folder_path).unwrap().filter_map(|e| e.ok()).collect();
    paths.retain(|path| fs::metadata(path.path()).unwrap().is_file());
    paths.retain(|path| vec_contains(&filetypes, path.path().extension().unwrap().to_str().unwrap()));
    paths.sort_by_key(|path| Reverse(modified_duration(sys_time, &path.path())));
    let mut file_count = file_counter(&paths)?; // try out the naming operation to see how many files it renames
    let lead_zeros = lead_zeros(5, file_count.1); // want to make sure that we have enough padding
    let mut files_renamed: u32 = 0;
    for path in paths {
        let file = path.path();
        let file_name = file.file_name().unwrap().to_str().unwrap().to_string();
        let mut ancestors = file.ancestors();
        ancestors.next();
        let directory = ancestors.next().unwrap().file_stem().unwrap().to_str().unwrap().replace(" ", "_");
        if file_name.starts_with(&directory) {
            continue
        } else {
            let new_file_name = directory + "_" + &zfill(file_count.0.to_string(), lead_zeros) + "_" + &file_name;
            println!("{} -> {}", file_name, new_file_name);
            fs::rename(file, new_file_name)?;
            file_count.0 += 1;
            files_renamed += 1;
        }
    }
    println!("Renamed {} files in {}", files_renamed, folder_path.display());
    return Ok(true)
}

/// Counts the files to be renamed. Some files may already have the directory name already prepended so no rename needs to be done.
fn file_counter(paths: &Vec<fs::DirEntry>) -> Result<(u32, u32),  Box<dyn Error>> {
    let mut files: u32 = 0;
    let mut files_already_modified: u32 = 0;
    for path in paths {
        let file = path.path();
        let file_name = file.file_name().unwrap().to_str().unwrap().to_string();
        let mut ancestors = file.ancestors();
        ancestors.next();
        let directory = ancestors.next().unwrap().file_stem().unwrap().to_str().unwrap().replace(" ", "_");
        if file_name.starts_with(&directory) {
            files_already_modified += 1;
        } 
        files += 1;
    }
    return Ok((files_already_modified, files))
}

/// Reads a text file into a String and parses it into a vector containing the filetypes.
/// 
/// "#" and "//" can be used as comments in the file
/// # Example Filetypes File Setup
/// // Comment
///
/// \# Comment
///
/// .filetype1
///
/// .filetype2
///
/// .filetype3
fn get_filetypes(filetypes_file: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut contents = fs::read_to_string(filetypes_file)?;
    contents = contents.to_ascii_lowercase() + &contents.to_ascii_uppercase();
    let mut contents_vec: Vec<String> = contents.split_whitespace().map(str::to_string).collect();
    contents_vec.retain(|entry| entry.starts_with("."));
    contents_vec.retain(|entry| !entry.contains("#"));
    contents_vec.retain(|entry| !entry.contains("//"));
    return Ok(contents_vec)
}

/// Uses a default list of filetypes. The default list is read in as a String to keep this function similar to the main get_filetypes function.
fn alt_get_filetypes(contents: String) -> Result<Vec<String>, Box<dyn Error>> {
    let expanded_contents = contents.to_ascii_lowercase() + &contents.to_ascii_uppercase();
    let mut contents_vec: Vec<String> = expanded_contents.split_whitespace().map(str::to_string).collect();
    contents_vec.retain(|entry| entry.starts_with("."));
    contents_vec.retain(|entry| !entry.contains("#"));
    return Ok(contents_vec)
}

/// Creates and returns a String of length new_length with leading zeros.
///
/// If the string is already of length or larger new_length then the original String is returned.  
fn zfill(str: String, new_length: usize) -> String {
    let mut new_string: String = str.to_owned();
    if str.chars().count() < new_length {
        let mut index = 0;
        while new_string.chars().count() < new_length {
            new_string.insert(index, '0');
            index += 1;
        }
    }
    return new_string;
}

/// Checks to make sure that a situation where the length of the string with leading zeros can support the amount of files in the directory.
fn lead_zeros(mut lead_zeros: usize, file_count: u32) -> usize {
    if file_count.to_string().len() >= lead_zeros {
        lead_zeros += 2;
    }
    return lead_zeros
}

/// This is used since option_result_contains for vectors is unstable. This checks is a vector made of Strings contains a string. 
fn vec_contains(vec: &Vec<String>, str: &str) -> bool {    
    let mut new_string = String::from(str);
    new_string.insert(0, '.');
    let mut contains = false;
    for element in vec {
        if *element == new_string {
            contains = true;
        }
    }
    return contains
}

/// Returns how long ago a file was modified. A time to compare has to be provided to ensure that all comparisions are compared to the same time.
/// # Note
/// Use of unwrap() is intentional since we want to panic if file modified time cannot be found.
/// If modified time is incorrect this will cause the files to be renamed in the incorrect order!
fn modified_duration(time: std::time::SystemTime, file: &std::path::Path) -> u128 {
    let modified_time = fs::metadata(file).unwrap().modified();
    let duration = time.duration_since(modified_time.unwrap());
    return duration.unwrap().as_millis()
}