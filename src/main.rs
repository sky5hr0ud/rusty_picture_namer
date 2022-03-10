use std::error::Error;
use std::fs;
use std::time::SystemTime;
use std::cmp::Reverse;
use std::env;
use walkdir::WalkDir;

fn main() {
    if env::args().len() < 2 {
        println!("Need two args! <folder_path> <list_of_filetypes>");
        std::process::exit(1);
    }
    let folder_path = env::args().nth(1).unwrap();
    let filetypes_path = env::args().nth(2).unwrap();
    let result = directory_walker(&folder_path, &filetypes_path);
    println!("{:?}", result);
    std::process::exit(0);
}

fn directory_walker(folder_path: &str, filetypes_path: &str) -> Result<bool, Box<dyn Error>> {
    println!("Preparing to rename files in {}", folder_path);
    let filetypes = get_filetypes(filetypes_path)?;
    let mut directories: Vec<walkdir::DirEntry> = WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()).collect();
    directories.retain(|entry| fs::metadata(entry.path()).unwrap().is_dir());
    for directory in directories {
        file_namer(directory.path(), &filetypes)?;
    }
    return Ok(true)
}

fn file_namer(folder_path: &std::path::Path, filetypes: &Vec<String>) -> Result<bool, Box<dyn Error>> {
    std::env::set_current_dir(folder_path)?;
    let sys_time = SystemTime::now();
    let mut paths: Vec<fs::DirEntry> = fs::read_dir(folder_path).unwrap().filter_map(Result::ok).collect();
    paths.retain(|path| fs::metadata(path.path()).unwrap().is_file());
    paths.retain(|path| vec_contains(&filetypes, path.path().extension().unwrap().to_str().unwrap()));
    paths.sort_by_key(|path| Reverse(sys_time.duration_since(fs::metadata(path.path()).unwrap().modified().unwrap()).unwrap().as_millis()));
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

/// Count the files to be renamed. Some files may already have the directory name already prepended so no rename needs to be done.
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

fn get_filetypes(filetypes_file: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut contents = fs::read_to_string(filetypes_file)?;
    contents = contents.to_ascii_lowercase() + &contents.to_ascii_uppercase();
    let mut contents_vec: Vec<String> = contents.split_whitespace().map(str::to_string).collect();
    contents_vec.retain(|entry| entry.starts_with("."));
    contents_vec.retain(|entry| !entry.contains("#"));
    return Ok(contents_vec)
}

/// Returns a String of length new_length with leading zeros. 
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

fn lead_zeros(mut lead_zeros: usize, file_count: u32) -> usize {
    if file_count.to_string().len() >= lead_zeros {
        lead_zeros += 2;
    }
    return lead_zeros
}

// option_result_contains is unstable. 
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