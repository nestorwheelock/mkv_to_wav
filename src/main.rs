use std::env;
use std::fs::{self, File};
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use filetime::{set_file_times, FileTime};
use rayon::prelude::*;

fn create_audio_directory() -> Result<PathBuf> {
    let audio_dir = env::current_dir()?.join("audio");
    if !audio_dir.exists() {
        fs::create_dir(&audio_dir)?;
    }
    Ok(audio_dir)
}

fn preserve_timestamps(src: &Path, dest: &Path) -> Result<()> {
    let metadata = fs::metadata(src)?;
    let accessed = metadata.accessed()?;
    let modified = metadata.modified()?;

    let accessed_time = FileTime::from_system_time(accessed);
    let modified_time = FileTime::from_system_time(modified);

    set_file_times(dest, accessed_time, modified_time)?;

    println!(
        "Preserved timestamps for {:?}: accessed {:?}, modified {:?}",
        dest,
        DateTime::<Local>::from(accessed),
        DateTime::<Local>::from(modified)
    );

    Ok(())
}

fn convert_to_wav(file_path: PathBuf) -> Result<PathBuf> {
    let audio_dir = create_audio_directory()?;

    // Generate the output .wav file name
    let file_stem = file_path.file_stem().unwrap().to_str().unwrap();
    let output_wav = audio_dir.join(format!("{}.wav", file_stem));

    // Use ffmpeg to convert the file to .wav, stripping video
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&file_path)
        .arg("-vn")
        .arg(&output_wav)
        .status()?;

    if status.success() {
        println!("Converted {:?} to {:?}", file_path, output_wav);
        preserve_timestamps(&file_path, &output_wav)?;
    } else {
        eprintln!("Failed to convert {:?}", file_path);
    }

    Ok(output_wav)
}

fn remove_file(file_path: PathBuf) -> Result<()> {
    fs::remove_file(&file_path)?;
    println!("Removed file: {:?}", file_path);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let directory = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    let entries: Vec<PathBuf> = fs::read_dir(&directory)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("mkv"))
        .collect();

    entries.par_iter().for_each(|file_path| {
        if let Err(e) = convert_to_wav(file_path.to_path_buf()) {
            eprintln!("Error processing file {:?}: {:?}", file_path, e);
        }
    });

    Ok(())
}

