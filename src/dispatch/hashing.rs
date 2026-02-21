use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use xxhash_rust::xxh3::Xxh3;

use crate::errors::RobeError;

pub fn are_paths_equal(p1: &Path, p2: &Path) -> Result<bool, RobeError> {
    if p1.is_dir() && p2.is_dir() {
        dirs_equal_parallel(p1, p2)
    } else {
        files_equal(p1, p2)
    }
}

fn dirs_equal_parallel(dir1: &Path, dir2: &Path) -> Result<bool, RobeError> {
    let mut entries1: Vec<PathBuf> = std::fs::read_dir(dir1)?
        .map(|e| e.expect("").path())
        .collect();

    let mut entries2: Vec<PathBuf> = std::fs::read_dir(dir2)?
        .map(|e| e.expect("").path())
        .collect();

    if entries1.len() != entries2.len() {
        return Ok(false);
    }

    entries1.sort();
    entries2.sort();

    let results: Result<Vec<bool>, RobeError> = entries1
        .par_iter()
        .zip(entries2.par_iter())
        .map(|(p1, p2)| compare_entry(p1, p2))
        .collect();

    Ok(results?.into_iter().all(|x| x))
}

fn compare_entry(p1: &PathBuf, p2: &PathBuf) -> Result<bool, RobeError> {
    if let Some(name1) = p1.file_name()
        && let Some(name2) = p2.file_name()
    {
        if name1 != name2 {
            return Ok(false);
        }

        if p1.is_dir() && p2.is_dir() {
            dirs_equal_parallel(p1, p2)
        } else if p1.is_file() && p2.is_file() {
            files_equal(p1, p2)
        } else {
            Ok(false)
        }
    } else {
        Err(RobeError::Hashing(format!(
            "Error obtaining file names: {:?}, {:?}",
            p1, p2
        )))
    }
}

fn files_equal(file1: &Path, file2: &Path) -> Result<bool, RobeError> {
    let meta1 = std::fs::metadata(file1)?;
    let meta2 = std::fs::metadata(file2)?;
    if meta1.len() != meta2.len() {
        return Ok(false);
    }

    let mut f1 = BufReader::new(File::open(file1)?);
    let mut f2 = BufReader::new(File::open(file2)?);
    let mut buffer1 = [0u8; 8192];
    let mut buffer2 = [0u8; 8192];

    loop {
        let n1 = f1.read(&mut buffer1)?;
        let n2 = f2.read(&mut buffer2)?;
        if n1 != n2 {
            return Ok(false);
        }
        if n1 == 0 {
            break;
        }

        let mut h1 = Xxh3::new();
        h1.update(&buffer1[..n1]);
        let mut h2 = Xxh3::new();
        h2.update(&buffer2[..n2]);
        if h1.digest() != h2.digest() {
            return Ok(false);
        }
    }

    Ok(true)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self};
    use tempfile::{TempDir, tempdir};

    fn create_dirs() -> (TempDir, TempDir) {
        let tmp1 = tempdir().unwrap();
        let tmp2 = tempdir().unwrap();
        (tmp1, tmp2)
    }

    #[test]
    fn test_identical_text_files() -> Result<(), RobeError> {
        let (tmp1, tmp2) = create_dirs();
        let dir1 = tmp1.path();
        let dir2 = tmp2.path();

        let f1 = dir1.join("file1.txt");
        let f2 = dir2.join("file1.txt");
        fs::write(&f1, b"Hello, world!")?;
        fs::write(&f2, b"Hello, world!")?;

        assert!(are_paths_equal(dir1, dir2)?);
        Ok(())
    }

    #[test]
    fn test_different_text_files() -> Result<(), RobeError> {
        let (tmp1, tmp2) = create_dirs();
        let dir1 = tmp1.path();
        let dir2 = tmp2.path();

        let f1 = dir1.join("file1.txt");
        let f2 = dir2.join("file1.txt");
        fs::write(&f1, b"Hello")?;
        fs::write(&f2, b"Goodbye")?;

        assert!(!are_paths_equal(dir1, dir2)?);
        Ok(())
    }

    #[test]
    fn test_identical_binary_files() -> Result<(), RobeError> {
        let (tmp1, tmp2) = create_dirs();
        let dir1 = tmp1.path();
        let dir2 = tmp2.path();

        let bin1 = dir1.join("binary.bin");
        let bin2 = dir2.join("binary.bin");
        let data = vec![0u8, 255, 128, 64, 32];
        fs::write(&bin1, &data)?;
        fs::write(&bin2, &data)?;

        assert!(are_paths_equal(dir1, dir2)?);
        Ok(())
    }

    #[test]
    fn test_different_binary_files() -> Result<(), RobeError> {
        let (tmp1, tmp2) = create_dirs();
        let dir1 = tmp1.path();
        let dir2 = tmp2.path();

        let bin1 = dir1.join("binary.bin");
        let bin2 = dir2.join("binary.bin");
        let mut data = vec![0u8, 255, 128, 64, 32];
        fs::write(&bin1, &data)?;
        data[0] = 1;
        fs::write(&bin2, &data)?;

        assert!(!are_paths_equal(dir1, dir2)?);
        Ok(())
    }

    #[test]
    fn test_nested_directories_equal() -> Result<(), RobeError> {
        let (tmp1, tmp2) = create_dirs();
        let dir1 = tmp1.path();
        let dir2 = tmp2.path();

        let sub1 = dir1.join("sub");
        let sub2 = dir2.join("sub");
        fs::create_dir(&sub1)?;
        fs::create_dir(&sub2)?;
        fs::write(sub1.join("nested.txt"), b"Nested content")?;
        fs::write(sub2.join("nested.txt"), b"Nested content")?;

        assert!(are_paths_equal(dir1, dir2)?);
        Ok(())
    }

    #[test]
    fn test_nested_directories_different() -> Result<(), RobeError> {
        let (tmp1, tmp2) = create_dirs();
        let dir1 = tmp1.path();
        let dir2 = tmp2.path();

        let sub1 = dir1.join("sub");
        let sub2 = dir2.join("sub");
        fs::create_dir(&sub1)?;
        fs::create_dir(&sub2)?;
        fs::write(sub1.join("nested.txt"), b"Nested content")?;
        fs::write(sub2.join("nested.txt"), b"Different content")?;

        assert!(!are_paths_equal(dir1, dir2)?);
        Ok(())
    }

    #[test]
    fn test_multiple_files_parallel() -> Result<(), RobeError> {
        let (tmp1, tmp2) = create_dirs();
        let dir1 = tmp1.path();
        let dir2 = tmp2.path();

        for i in 0..50 {
            fs::write(dir1.join(format!("file{}.txt", i)), b"Some content")?;
            fs::write(dir2.join(format!("file{}.txt", i)), b"Some content")?;
        }

        assert!(are_paths_equal(dir1, dir2)?);
        Ok(())
    }
}
