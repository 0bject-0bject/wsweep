/// 
/// Copyright 2023, [object Object]
/// Licensed under MIT
///
/// Warnings:
/// Please check every file before deleting it, i am not responsible for any damage caused by this tool!
/// 

use std::{
    path::{PathBuf, Path}, 
    time::SystemTime, thread,
    io::{self},
    sync::mpsc::{channel, Sender}, fs, 
};

use crate::{colors::Colors, same_line_input};

// last modified time
fn get_age(file: &Path) -> io::Result<f64> {
    let metadata = file.metadata()?;
    let modified = metadata.modified()
        .unwrap_or_else(|_| SystemTime::now())
        .elapsed()
        .map(|duration| duration.as_secs_f64() / 60.0 / 60.0 / 24.0)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    Ok(modified)
}



fn check_and_delete(path: PathBuf, dry_run: bool) -> io::Result<()> {
    // Prompt the user for confirmation before deleting
    let input = same_line_input(&format!("{} {}?(y/n)", "Delete".bright_red(), path.display()));

    // Check if the user confirmed deletion and if the file or directory exists
    if input.trim() == "y" || input.trim() == "yes" && path.exists() {
        if dry_run {
            // Perform a dry run!
            println!("{} {} {} have been deleted", "Dry run:".bright_green(), path.display(), " test ".bold_red());
        } else {
            // Spawn a new thread to perform the deletion
            let handle = thread::spawn(move || {
                if path.is_dir() {
                    // Delete the directory and its contents
                    fs::remove_dir_all(&path)?;
                    println!("{} {}", "Deleted:".bright_green(), path.display())
                } else if path.is_file() {
                    // Delete the file
                    fs::remove_file(&path)?;
                    println!("{} {}", "Deleted:".bright_green(), path.display())
                }
                Ok(()) as Result<(), io::Error>
            });
            // Wait for the deletion thread to finish
            handle.join().unwrap()?;
        }
    }
    Ok(())
}

/// Recursively scans the given directory for files and directories that can be deleted, and sends them to the given `Sender`.
fn scan_directory_worker(path: PathBuf, tx: Sender<PathBuf>, verbose: bool) {
    let deletable_files = vec![
        "package-lock.json",
        "yarn.lock",
        "cargo.lock",
    ];
    let deletable_dirs = vec![
        "node_modules",
        "target",
    ];

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if verbose {
                // If Verbose, print a message indicating that the directory is being scanned
                println!("{} {:?}", "Scanning:".bright_cyan(), path);
            }

            if deletable_dirs.contains(&path.file_name().unwrap().to_str().unwrap()) {
                // Send the directory path to the channel if it can be deleted
                tx.send(path).unwrap();
            } else {
                // Recursively scan the subdirectory
                scan_directory_worker(path, tx.clone(), verbose);
            }
        } else {
            if verbose {
                // Print a message indicating that the file is being scanned
                println!("{} {:?}", "Scanning:".bright_cyan(), path);
            }

            if deletable_files.contains(&path.file_name().unwrap().to_str().unwrap()) {
                // Send the file path to the channel if it can be deleted
                tx.send(path).unwrap();
            }
        }
    }
}

/// Recursively scans the given directory for files and directories that can be deleted, and returns them as a vector.
fn find_unused_files(path: PathBuf, verbose: bool) -> Vec<PathBuf> {

    // Create a channel for comms between threads
    let (tx, rx) = channel();

    let thread = thread::spawn(move || {
        // Spawns a worker thread to scan the directory
        scan_directory_worker(path, tx, verbose);
    });

    let mut files = Vec::new();
    for path in rx {
        // Collect the paths sent by the worker threads
        files.push(path);
    }

    // Wait for the worker thread to finish
    thread.join().unwrap();

    files
}

pub fn start_deletion(path: PathBuf, dry_run: bool, verbose: bool, minimum_age: u64) -> io::Result<()> {
    // Find all the files and directories that can be deleted
    let deletable_files = find_unused_files(path, verbose);

    for file in deletable_files {
        // Check if the file is older than the minimum age requirement
        if get_age(&file)? < minimum_age as f64 {
            continue;
        }

        // Check and delete the file or directory
        check_and_delete(file, dry_run)?;
    }

    Ok(())
}