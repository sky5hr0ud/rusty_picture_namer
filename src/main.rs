use std::io::Error;
use std::fs;
use std::time::SystemTime;
use std::cmp::Reverse;

fn main() {
    println!("Hello, world!");
    let folder_path = "C:\\Users\\distu\\Downloads\\New folder\\Random 2022\\755018865683263.webp";
    println!("{:?}", file_namer(folder_path));
}

fn file_namer(folder_path: &str) -> Result<bool, Error> {
    std::env::set_current_dir(folder_path)?;
    let sys_time = SystemTime::now();
    let mut paths: Vec<fs::DirEntry> = fs::read_dir(folder_path).unwrap().filter_map(Result::ok).collect();
    paths.retain(|path| fs::metadata(path.path()).unwrap().is_file());
    let file_extensions = ["jpg", "png", "gif"];
    paths.retain(|path| {path.path().extension().unwrap().to_str().map_or(true, |file_extension| file_extensions.contains(&file_extension))});
    paths.sort_by_key(|path| Reverse(sys_time.duration_since(fs::metadata(path.path()).unwrap().modified().unwrap()).unwrap().as_millis()));
    let mut file_count = file_counter(&paths)?; // try out the naming operation to see how many files it renames
    let lead_zeros = lead_zeros(5, file_count); // want to make sure that we have enough padding
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
            let new_file_name = directory + "_" + &zfill(file_count.to_string(), lead_zeros) + "_" + &file_name;
            println!("{} -> {}", file_name, new_file_name);
            fs::rename(file, new_file_name)?;
            file_count += 1;
            files_renamed += 1;
        }
    }
    println!("Renamed {} files in {}", files_renamed, folder_path);
    return Ok(true)
}

/// Count the files to be renamed. Some files may already have the directory name already prepended so no rename needs to be done.
fn file_counter(paths: &Vec<fs::DirEntry>) -> Result<u32, Error> {
    let mut count: u32 = 0;
    for path in paths {
        let file = path.path();
        let file_name = file.file_name().unwrap().to_str().unwrap().to_string();
        let mut ancestors = file.ancestors();
        ancestors.next();
        let directory = ancestors.next().unwrap().file_stem().unwrap().to_str().unwrap().replace(" ", "_");
        if file_name.starts_with(&directory) {
            continue
        } else {
            count += 1;
        }
    }
    return Ok(count)
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
    } else {
        lead_zeros = file_count as usize;
    }
    return lead_zeros
}