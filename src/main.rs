mod input;
use std::os::unix::fs::{MetadataExt};
use structopt::StructOpt;

struct Directory {
  paths: Vec<File>,
}

enum DirSortType {
  Name,
  Created,
  Modified,
  Size,
  Not,
}

enum FileType {
  Dir,
  Symlink,
  File,
}

struct File {
  path: std::path::PathBuf,
  file_type: FileType,
}

impl FileType {
  fn new(file: &std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
    match file.symlink_metadata()?.is_dir() {
      true => Ok(Self::Dir),
      false => {
        match file.symlink_metadata()?.file_type().is_symlink() {
          true => Ok(Self::Symlink),
          false => Ok(Self::File)
        }
      }
    }
  }
}

impl File {
  fn new(file: std::path::PathBuf) -> Self {
    Self {
      file_type: FileType::new(&file).unwrap(),
      path: file,
    }
  }
}

fn get_sort_type(sort_t: [bool; 4]) -> DirSortType {
  for (i, t) in sort_t.iter().enumerate() {
    if *t {
      match i {
        0 => {
          return DirSortType::Name
        },
        1 => {
          return DirSortType::Created
        },
        2 => {
          return DirSortType::Modified
        },
        3 => {
          return DirSortType::Size
        },
        _ => ()
      }
    }
  }
  DirSortType::Not
}

impl Directory {
  fn new(dir: std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
    if !std::path::Path::new(&dir).exists() {
      let mut new_paths = Vec::new();
      let paths = std::fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<std::path::PathBuf>, std::io::Error>>()?;

      for p in paths {
        if p
          .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_lowercase()
            .contains(&dir.display().to_string().to_lowercase())
            {
              new_paths.push(File::new(p))
            }
      }
      if new_paths.is_empty() {
        println!("File could not be found");
        std::process::exit(1)
      }
      Ok (
        Self {
          paths: new_paths
        }
      )
    }
    else {
      let paths = std::fs::read_dir(dir)?
        .map(|res| res.map(|e| File::new(e.path()) ))
        .collect::<Result<Vec<File>, std::io::Error>>()?;
      Ok(
        Self {
          paths
        }
      )
    }
  }

  fn name_sort(&mut self) {
    self.paths.sort_by(|a, b| a.path.file_name().unwrap().to_str().unwrap().to_lowercase().cmp(&b.path.file_name().unwrap().to_str().unwrap().to_lowercase()))
  }

  fn create_sort(&mut self) {
    self.paths.sort_by(|a, b| a.path.symlink_metadata().unwrap().created().unwrap().cmp(&b.path.symlink_metadata().unwrap().created().unwrap()))
  }

  fn modified_sort(&mut self) {
    self.paths.sort_by(|a, b| a.path.symlink_metadata().unwrap().modified().unwrap().cmp(&b.path.symlink_metadata().unwrap().modified().unwrap()))
  }

  fn size_sort(&mut self) {
    self.paths.sort_by(|a, b| a.path.symlink_metadata().unwrap().size().cmp(&b.path.symlink_metadata().unwrap().size()))
  }

  fn sort_paths(&mut self) {
    match get_sort_type([input::Cli::from_args().name, input::Cli::from_args().created, input::Cli::from_args().modified, input::Cli::from_args().size]) {
      DirSortType::Name => { 
        self.name_sort();
      },
      DirSortType::Created => { 
        self.create_sort();
      },
      DirSortType::Modified => { 
        self.modified_sort();
      },
      DirSortType::Size => {
        self.size_sort();
      },
      DirSortType::Not => (),
    }
  }
}

impl std::fmt::Display for Directory {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    Ok(
      for i in &self.paths {
        write!(f, " {} ", i.path.file_name().unwrap().to_str().unwrap())?;
      }
    )
  }
}

fn main() {
  let mut dir = Directory::new(input::Cli::from_args().dir).unwrap();
  dir.sort_paths();
  println!("{}", dir)
}
