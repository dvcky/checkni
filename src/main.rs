// system imports
use std::convert::TryInto;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, stdin, stdout, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
// imported packages
use md5::{Md5, Digest};
use walkdir::WalkDir;
use zip_extensions::zip_extract;

// description for the program
fn description() {
    println!("\n================");
    println!("checkni (Check No-Intro) - version 0.1");
    println!("\\ 'chek-nē \\");
    println!(": a small program used for verifying game dumps with No-Intro");
    println!("https://github.com/dvcky/checkni");
    println!("================");
    println!("\nUsage: checkni <file or directory>\n");
}

fn log_checknifile(logfile: &mut File, gamefile: &CheckniFile) {
    write!(logfile, "File: {}\n", gamefile.path).unwrap();
    write!(logfile, "Hash: {}\n", gamefile.hash).unwrap();
    if gamefile.hash != "Empty file!" {
        if gamefile.find == "No match found!" {
            write!(logfile, "Find: {}\n", gamefile.find).unwrap();
        } else {
            write!(logfile, "Find: \"{}\"\n", gamefile.find).unwrap();
        }
    }
}

// struct used to connect a file to it's path, hash, and hopefully eventually a
// game name
struct CheckniFile {
    path: String,
    hash: String,
    find: String,
}

/* While this implementation may seem more complicated than it needs to be,
   there is good reason for it to be this way. Rather than loading the entire
   file straight into memory, which would only be about one line, checkni
   iteratively loads buffers of 64MB into the reader. This way, the program
   isn't attempting to use 4GB of memory on a 4GB file, for example.
*/ 
fn hash_file(file: &PathBuf) -> String {
    // create a hasher context (for storing and hashing data)
    let mut hasher = Md5::new();

    // create object to store 64MB buffer and a reader for the buffer
    let mut reader = BufReader::new(File::open(file).unwrap());
    let mut buffer = [0; 64*1024];

    // while buffer is not empty, add contents to hasher
    while reader.read(&mut buffer).unwrap() != 0 {
        hasher.update(buffer);
    }

    // format the hash into hexidecimal, since the md-5 package hashes in ascii???
    let mut hash = format!("{:x}", hasher.finalize());

    // check if empty hash. if so, replace with identifiable string.
    if hash == "d41d8cd98f00b204e9800998ecf8427e" {
        hash = String::from("Empty file!");
    }

    // finally, return hash
    return hash;
}

fn check_all_systems(sys_dir: &PathBuf, files: &mut Vec<CheckniFile>) {
    print!("[CHECKNI] Scanning database for matches...");
    stdout().flush().expect("");
    for sys_file in WalkDir::new(sys_dir).sort_by_key(|name| name.path().to_str().unwrap().to_lowercase()) {
        let sys_as_path = sys_file.unwrap().into_path();
        if sys_as_path.to_str().unwrap().ends_with(".dat") {
            check_system(&sys_as_path, files);
        }
    }
    println!("done!");
}

// scans a single system from a .dat file and parses it to find any file hashes
// that match the user's file hash batch
fn check_system(sys_path: &PathBuf, files: &mut Vec<CheckniFile>) {
    // initialize temporary variables
    let mut temp_find = "";
    let mut temp_hash;
    // read file for system, then create an xml parsing object for it
    let sys_string = fs::read_to_string(sys_path).unwrap();
    let sys_xml = roxmltree::Document::parse(sys_string.as_str()).unwrap();
    // go through file, looking for certain attributes
    for node in sys_xml.descendants() {
        if node.is_element() {
            // if attribute is "game", store the name of it in the case that we
            // find a matching hash
            if node.tag_name().name() == "game" {
                temp_find = node.attribute("name").unwrap();
            }
            // otherwise if it is an md5 hash for a rom, check if any of our files
            // have a matching hash. if they do, give them the game name.
            else if node.tag_name().name() == "rom" && node.has_attribute("md5") {
                temp_hash = node.attribute("md5").unwrap();
                for file in &mut *files {
                    if file.find == "No match found!" && file.hash != "Empty file!" && file.hash == temp_hash {
                        file.find = temp_find.to_string();
                    }
                }
            }
        }
    }
}

// returns the number of digits a integer has
fn get_digits(number: i32) -> i32 {
    let mut digits = 0;
    let mut temp_num = number;
    while temp_num > 0 {
        temp_num /= 10;
        digits += 1;
    }
    return digits;
}
// returns a string a spaces that pads the current number to the length of the total
fn get_padding(curr_number: i32, total_number: i32) -> String {
    let mut padding = String::new();
    for _ in 0..(get_digits(total_number) - get_digits(curr_number)) {
        padding += " ";
    }
    return padding
}

// small function for getting user input
fn prompt_input(prompt: &str) -> String {
    let mut line = String::new();
    print!("{}", prompt);
    stdout().flush().expect("");
    stdin().read_line(&mut line).unwrap();
    return line;
}

// main method
fn main() {
    // collect arguments, handle them with args.rs???
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            // DESCRIBE PROGRAM
            description();
        }
        2 => {
            // convert the second argument into a path. if it is a real path, continue!
            let path = Path::new(&args[1]);
            if path.exists() {

                // create an array to store file objects in
                let mut files: Vec<CheckniFile> = Vec::new();

                // get directory of checkni binary
                let mut checkni_dir = env::current_exe().unwrap();
                checkni_dir.pop();

                // create suspected path for database and zip file
                let db_dir = checkni_dir.join("db");
                let db_logs = checkni_dir.join("logs");
                let db_zip = checkni_dir.join("db.zip");
                
                // check for database. if doesn't exist, ask for zip
                // and extract it to the proper location
                if db_dir.is_dir() {
                    println!("[CHECKDB] Local copy of No-Intro database found!");
                } else if db_zip.is_file() {
                    print!("[CHECKDB] Database zip found! Extracting...");
                    stdout().flush().expect("");
                    zip_extract(&db_zip, &db_dir).unwrap();
                    println!("done!");
                    if prompt_input("[CHECKDB] Would you like to delete the zip file? (y/*): ") == "y\n" {
                        fs::remove_file(db_zip).unwrap();
                        println!("[CHECKDB] File deleted!");
                    }
                } else {
                    println!("[CHECKDB] No local copy of the No-Intro database found, please follow the instructions given on the project's repository!");
                    return;
                }

                // get current epoch and make a log file name from it
                let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();
                let logfile_name = timestamp.to_string() + &".log".to_string();
                // create logs folder if it doesn't exist yet
                fs::create_dir_all(&db_logs).unwrap();

                // create the log file and write out header content
                let mut logfile = File::create(db_logs.join(&logfile_name)).unwrap();
                write!(logfile, "checkni - version 0.1\n").unwrap();
                write!(logfile, "Started @: {}\n", &timestamp).unwrap();
                write!(logfile, "Scan type: ").unwrap();

                if path.is_file() {
                    // if the above is true, run a single file check
                    write!(logfile, "File\n\n").unwrap();

                    // convert argument into file
                    let mut singleton = PathBuf::new();
                    singleton.push(&args[1]);

                    // hash file and add it to the files vector
                    println!("[HASHING] {}", path.display());
                    files.push(CheckniFile {
                        path: singleton.to_path_buf().to_str().unwrap().to_string(),
                        hash: hash_file(&singleton),
                        find: "No match found!".to_string()
                    });
                    check_all_systems(&db_dir, &mut files);
                    log_checknifile(&mut logfile, &files[0]);
                    println!("[COMPLETE] Scan complete! Check the log file for results!\n\nFile generated: {}\n", db_logs.join(&logfile_name).display());
                } else {
                    write!(logfile, "Folder\n\n").unwrap();
                    // push files to a vector. we do this to order them alphebetically, but also so that we can get a file count
                    let mut ordered_files: Vec<PathBuf> = Vec::new();
                    for entry in WalkDir::new(path).sort_by_key(|name| name.path().to_str().unwrap().to_lowercase()) {
                        let entry = entry.unwrap();
                        let entry_pb = entry.into_path();
                        if entry_pb.is_file() {
                            ordered_files.push(entry_pb);
                        }
                    }
                    let mut curr_file = 0;
                    let num_files: i32 = ordered_files.len().try_into().unwrap();
                    write!(logfile, "TotalFile: {}\n", &num_files.to_string()).unwrap();
                    // hash each file and push it to the vector
                    for entry in ordered_files {
                        curr_file += 1;
                        let padding = get_padding(curr_file, num_files);
                        println!("[HASHING {}{}/{}] {}", padding, curr_file, num_files, entry.strip_prefix(path).expect("").display());
                        files.push(CheckniFile {
                            path: entry.to_str().unwrap().to_string(),
                            hash: hash_file(&entry),
                            find: "No match found!".to_string()
                        });
                    }

                    check_all_systems(&db_dir, &mut files);

                    // Count number of found files to logging
                    let mut found_files = 0;
                    for entry in &files {
                        if entry.find != "No match found!" && entry.hash != "Empty file!" {
                            found_files += 1;
                        }
                    }

                    // Log number of found files, as well as specific results
                    write!(logfile, "FoundFile: {}\n", &found_files.to_string()).unwrap();
                    write!(logfile, "================================\n").unwrap();
                    for entry in &files {
                        log_checknifile(&mut logfile, entry);
                        write!(logfile, "================================\n").unwrap();
                    }
                    println!("[COMPLETE] Scan complete! Check the log file for results!\n\nFile generated: {}\n", db_logs.join(&logfile_name).display());
                }
            } else {
                println!("\nInvalid file/directory path! Please read usage below.");
                description();
            }
        }
        _ => {
            println!("\nMore arguments than expected! Please read usage below.");
            description();
        }
    }
}