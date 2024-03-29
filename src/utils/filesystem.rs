use std::{fs, io::{self, Error, BufReader}, path::PathBuf};

pub fn open_file(path: String) -> Result<BufReader<fs::File>, Error> {
  let file = fs::File::open(path)?;
  return Ok(BufReader::new(file));
}

pub fn open_folder(path: String) -> Result<Vec<PathBuf>, Error> {
  let mut entries = fs::read_dir(path)? 
      .map(|res| res.map(|e| e.path()))
      .collect::<Result<Vec<_>, io::Error>>()?;
  entries.sort();
  Ok(entries)
}

pub fn open_folder_or_create(path: String) -> Result<Vec<PathBuf>, Error> {
  match open_folder(path.clone()) {
    Ok(entries) => Ok(entries),
    Err(e) => match e.kind() {
      io::ErrorKind::NotFound => match create_folder(path.clone()) {
        Ok(()) => Ok(Vec::new()),
        Err(e) => Err(e),
      },
      _ => Err(e),
    }
  }
}

pub fn create_file(path: String, contents: String) -> Result<(), Error> {
  fs::write(path, contents)
}

pub fn create_folder(path: String) -> Result<(), Error> {
  fs::create_dir_all(path)
}

pub fn delete_folder(path: String) -> Result<(), Error> {
  fs::remove_dir_all(path)
}
