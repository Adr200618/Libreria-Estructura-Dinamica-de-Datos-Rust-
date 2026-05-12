use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub enum FileError {
    IoError(io::Error),
    InvalidPath(String),
    FileNotFound(String),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::IoError(e) => write!(f, "Error de E/S: {}", e),
            FileError::InvalidPath(p) => write!(f, "Ruta inválida: {}", p),
            FileError::FileNotFound(p) => write!(f, "Archivo no encontrado: {}", p),
        }
    }
}

pub fn read_file(path: &str) -> Result<String, FileError> {
    let path_obj = Path::new(path);
    
    if !path_obj.exists() {
        return Err(FileError::FileNotFound(path.to_string()));
    }
    
    fs::read_to_string(path_obj)
        .map_err(|e| FileError::IoError(e))
}

pub fn read_file_lines(path: &str) -> Result<Vec<String>, FileError> {
    let path_obj = Path::new(path);
    
    if !path_obj.exists() {
        return Err(FileError::FileNotFound(path.to_string()));
    }
    
    let file = File::open(path_obj).map_err(|e| FileError::IoError(e))?;
    let reader = BufReader::new(file);
    
    let lines: Result<Vec<String>, _> = reader.lines().collect();
    lines.map_err(|e| FileError::IoError(e))
}

pub fn write_file(path: &str, content: &str) -> Result<(), FileError> {
    let path_obj = Path::new(path);
    
    let mut file = File::create(path_obj).map_err(|e| FileError::IoError(e))?;
    file.write_all(content.as_bytes()).map_err(|e| FileError::IoError(e))?;
    
    Ok(())
}

pub fn append_to_file(path: &str, content: &str) -> Result<(), FileError> {
    let path_obj = Path::new(path);
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path_obj)
        .map_err(|e| FileError::IoError(e))?;
    
    file.write_all(content.as_bytes()).map_err(|e| FileError::IoError(e))?;
    writeln!(file).map_err(|e| FileError::IoError(e))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    
    #[test]
    fn test_write_and_read() {
        let temp_file = temp_dir().join("test.txt");
        let path = temp_file.to_str().unwrap();
        
        let content = "Hola mundo";
        assert!(write_file(path, content).is_ok());
        
        let read_content = read_file(path).unwrap();
        assert_eq!(read_content, content);
        
        std::fs::remove_file(path).unwrap();
    }
}