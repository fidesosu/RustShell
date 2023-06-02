use std::io::{self, Write, BufWriter};
use std::{fs::{File, self}};
use std::path::{Path};
use std::time::{Instant};
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub fn scan() {
    let file_path = Path::new("files.txt");
    
    if file_path.exists() {
        println!("        File already exists!");
        println!("        Do you want to overwrite it? (y/n)");
        print!("     scan > ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        println!();
        
        if input == "y" {
            fs::remove_file(file_path).unwrap();
            println!("        Removed: {}", file_path.display());
            create_file_and_scan(file_path);
        } else {
            println!("        Aborting...");
        }
    } else {
        create_file_and_scan(file_path);
    }
}

fn create_file_and_scan(file_path: &Path) {
    let dir = Path::new("C:\\");
    
    println!("        Creating new file...");
    let file = File::create(file_path).unwrap();
    let file_writer = Arc::new(Mutex::new(BufWriter::new(file)));
    println!("        Created file: {}", file_path.display());
    
    println!();
    println!("        This Might Take A While...");
    
    let start_time = Instant::now();
    let counter = Arc::new(Mutex::new(0));
    let pool = ThreadPool::new(8); // Number of worker threads in the thread pool
    scan_dir_parallel(&dir, &counter, Arc::clone(&file_writer), &pool);
    pool.join();
    
    file_writer.lock().unwrap().flush().unwrap();
    let elapsed_time = start_time.elapsed();
    
    let total_time = elapsed_time.as_secs_f64();
    let files_per_second = (counter.lock().unwrap().clone() as f64 / total_time).round();
    
    println!();
    println!("        Scan Complete!");
    println!("        Total Time: {:.2} seconds", total_time);
    println!("        Files Per Second: {}", files_per_second);
}

fn scan_dir_parallel(
    dir: &Path,
    counter: &Arc<Mutex<usize>>,
    file_writer: Arc<Mutex<BufWriter<File>>>,
    pool: &ThreadPool,
) {
    let blacklisted_dirs: [&str; 15] = [
        "C:\\Windows",
        "C:\\ProgramData\\Microsoft\\Windows\\Containers\\BaseImages",
        "C:\\Users\\All Users",
        "C:\\Documents and Settings",
        "C:\\ProgramData\\Application Data",
        "C:\\ProgramData\\Desktop",
        "C:\\ProgramData\\Documents",
        "C:\\ProgramData\\Start Menu",
        "C:\\ProgramData\\Templates",
        "C:\\Users\\Default",
        "C:\\$Recycle.Bin",
        "C:\\$WINDOWS.~BT",
        "C:\\$Windows.~WS",
        "C:\\$AV_ASW",
        "C:\\$SysReset",
    ];

    if !blacklisted_dirs.contains(&dir.to_string_lossy().as_ref()) {
        match fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_file() {
                        let file_writer = Arc::clone(&file_writer);
                        let counter = Arc::clone(counter);
                        pool.execute(move || {
                            file_writer
                                .lock()
                                .unwrap()
                                .write_all(format!("{}\n", path.to_string_lossy()).as_bytes())
                                .unwrap();
                            let mut count = counter.lock().unwrap();
                            *count += 1;
                        });
                    } else if path.is_dir() {
                        scan_dir_parallel(&path, counter, Arc::clone(&file_writer), pool);
                    }
                }
            }
            Err(e) => {
                let mut log_file = File::create("error.log").unwrap();
                writeln!(log_file, "Error reading directory {}: {}", dir.display(), e).unwrap();
            }
        }
    }
}