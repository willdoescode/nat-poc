#[derive(Debug)]
struct Directory {
  paths: Vec<std::path::PathBuf>,
}

enum DirSortType {
  Name,
  Created,
  Modified,
  Size,
  Not,
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
    let paths = std::fs::read_dir(dir)?
      .map(|res| res.map(|e| e.path()))
      .collect::<Result<Vec<std::path::PathBuf>, std::io::Error>>()?;
    Ok(
      Self {
        paths
      }
    )
  }

  fn sort_paths(&mut self) {
    match get_sort_type([true, false, false, false]) {
      DirSortType::Name => { 
        self.paths.sort_by(|a, b| a.file_name().unwrap().to_str().unwrap().to_lowercase().cmp(&b.file_name().unwrap().to_str().unwrap().to_lowercase()))
      },
      DirSortType::Created => { 
        self.paths.sort_by(|a, b| a.file_name().unwrap().to_str().unwrap().to_lowercase().cmp(&b.file_name().unwrap().to_str().unwrap().to_lowercase()))
      },
      DirSortType::Modified => { 
        self.paths.sort_by(|a, b| a.file_name().unwrap().to_str().unwrap().to_lowercase().cmp(&b.file_name().unwrap().to_str().unwrap().to_lowercase()))
      },
      DirSortType::Size => { 
        self.paths.sort_by(|a, b| a.file_name().unwrap().to_str().unwrap().to_lowercase().cmp(&b.file_name().unwrap().to_str().unwrap().to_lowercase()))
      },
      DirSortType::Not => { 
        self.paths.sort_by(|a, b| a.file_name().unwrap().to_str().unwrap().to_lowercase().cmp(&b.file_name().unwrap().to_str().unwrap().to_lowercase()))
      },
    }
  }
}

fn main() {
  let mut dir = Directory::new(std::path::PathBuf::from(".")).unwrap();
  dir.sort_paths();
  println!("{:?}", dir)
}
