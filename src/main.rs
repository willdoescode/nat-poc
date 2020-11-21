#![allow(dead_code)]
mod input;
mod text_effects;
use std::os::unix::fs::{MetadataExt, FileTypeExt};
use structopt::StructOpt;
use termion;

struct Directory {
  paths: Vec<File>,
  stdout: std::io::Stdout,
}

enum DirSortType {
  Name,
  Created,
  Modified,
  Size,
  Not,
}

#[derive(Clone)]
struct File {
  path: std::path::PathBuf,
  file_type: Vec<PathType>,
}

#[derive(Copy, Clone, Debug)]
enum PathType {
  Dir,
  Symlink,
  Path,
  Pipe,
  CharD,
  BlockD,
  Socket,
}

impl PathType {
  fn new(file: &std::path::PathBuf) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
    let mut return_val = Vec::new();
    if file.symlink_metadata()?.is_dir() {
      return_val.push(Self::Dir)
    } 

    if file.symlink_metadata()?.file_type().is_symlink() {
      return_val.push(Self::Symlink)
    }

    if file.symlink_metadata()?.file_type().is_fifo() {
      return_val.push(Self::Pipe)
    } 

    if file.symlink_metadata()?.file_type().is_char_device() {
      return_val.push(Self::CharD)
    }

    if file.symlink_metadata()?.file_type().is_block_device() {
      return_val.push(Self::BlockD)
    }

    if file.symlink_metadata()?.file_type().is_socket() {
      return_val.push(Self::Socket)
    } 

    if return_val.is_empty() {
      return_val.push(Self::Path)
    }

    Ok(return_val)
  }

  fn get_color_for_type(&self) -> String {
    match self {
      Self::Dir => format!("{}", termion::color::Fg(termion::color::LightBlue)),
      Self::Symlink => format!("{}", termion::color::Fg(termion::color::LightMagenta)),
      Self::Path => format!("{}", termion::color::Fg(termion::color::White)),
      Self::Pipe => format!("{}", termion::color::Fg(termion::color::Yellow)),
      Self::CharD => format!("{}", termion::color::Fg(termion::color::LightGreen)),
      Self::BlockD => format!("{}", termion::color::Fg(termion::color::LightGreen)),
      Self::Socket => format!("{}", termion::color::Fg(termion::color::LightGreen)),
    }
  }

  fn get_text_traits_for_type(&self, name: &str, file: &std::path::PathBuf) -> String {
    match self {
      Self::Dir => text_effects::bold(&format!("{}/", name)),
      Self::Symlink => text_effects::italic(&format!("{} -> {}", name, std::fs::canonicalize(std::fs::read_link(file).unwrap()).unwrap_or(file.clone()).to_str().unwrap_or(name))),
      Self::Path => text_effects::bold(name),
      Self::Pipe => text_effects::bold(&format!("{}|", name)),
      Self::CharD => text_effects::bold(name),
      Self::BlockD => text_effects::bold(name),
      Self::Socket => text_effects::bold(name),
    }
  }
}

impl File {
  fn new(file: std::path::PathBuf) -> Self {
    Self {
      file_type: PathType::new(&file).unwrap(),
      path: file,
    }
  }
}

fn get_sort_type(sort_t: [bool; 4]) -> DirSortType {
  for (i, t) in sort_t.iter().enumerate() {
    if *t {
      match i {
        0 => return DirSortType::Name,
        1 => return DirSortType::Created,
        2 => return DirSortType::Modified, 
        3 => return DirSortType::Size,
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
              let f = File::new(p);
              new_paths.push(f)
            }
      }

      if new_paths.is_empty() {
        println!("Path could not be found");
        std::process::exit(1)
      }

      Ok (
        Self {
          paths: new_paths,
          stdout: std::io::stdout(),
        }
      )
    } else {
      let paths = std::fs::read_dir(dir)?
        .map(|res| res.map(|e| File::new(e.path())))
        .collect::<Result<Vec<File>, std::io::Error>>()?;
      Ok(
        Self {
          paths,
          stdout: std::io::stdout(),
        }
      )
    }
  }

  fn self_name_sort(&mut self) {
    self.paths.sort_by(|a, b| a.path.file_name().unwrap().to_str().unwrap().to_lowercase().cmp(&b.path.file_name().unwrap().to_str().unwrap().to_lowercase()))
  }

  fn self_create_sort(&mut self) {
    self.paths.sort_by(|a, b| a.path.symlink_metadata().unwrap().created().unwrap().cmp(&b.path.symlink_metadata().unwrap().created().unwrap()))
  }

  fn self_modified_sort(&mut self) {
    self.paths.sort_by(|a, b| a.path.symlink_metadata().unwrap().modified().unwrap().cmp(&b.path.symlink_metadata().unwrap().modified().unwrap()))
  }

  fn self_size_sort(&mut self) {
    self.paths.sort_by(|a, b| a.path.symlink_metadata().unwrap().size().cmp(&b.path.symlink_metadata().unwrap().size()))
  }

  fn sort_directory_then_path(&mut self) {
    let new = &self.paths;
    let mut newer = Vec::new();
    let mut directories = Vec::new();
    for (i, f) in new.iter().enumerate() {
      if f.path.symlink_metadata().unwrap().is_dir() {
        directories.push(new[i].clone());
      } else {
        newer.push(new[i].clone())
      }
    } 

    match get_sort_type([input::Cli::from_args().name, input::Cli::from_args().created, input::Cli::from_args().modified, input::Cli::from_args().size]) {
      DirSortType::Name => {
        name_sort(&mut directories);
        name_sort(&mut newer)
      },
      DirSortType::Created => {
        create_sort(&mut directories);
        create_sort(&mut newer)
      },
      DirSortType::Modified => {
        modified_sort(&mut directories);
        modified_sort(&mut newer)
      },
      DirSortType::Size => {
        size_sort(&mut directories);
        size_sort(&mut newer)
      },
      DirSortType::Not => (),
    }

    directories.append(&mut newer);
    self.paths = directories; 
  }

  fn sort_paths(&mut self) {
    match get_sort_type([input::Cli::from_args().name, input::Cli::from_args().created, input::Cli::from_args().modified, input::Cli::from_args().size]) {
      DirSortType::Name => self.self_name_sort(),
      DirSortType::Created => self.self_create_sort(),
      DirSortType::Modified => self.self_modified_sort(),
      DirSortType::Size => self.self_size_sort(),
      DirSortType::Not => (),
    }
  }

  fn sort(&mut self) {
    match input::Cli::from_args().gdf {
      true => self.sort_directory_then_path(),
      false => self.sort_paths(),
    }
  }
}

fn name_sort(dir: &mut Vec<File>) {
  dir.sort_by(|a, b| a.path.file_name().unwrap().to_str().unwrap().to_lowercase().cmp(&b.path.file_name().unwrap().to_str().unwrap().to_lowercase()))
}

fn create_sort(dir: &mut Vec<File>) {
  dir.sort_by(|a, b| a.path.symlink_metadata().unwrap().created().unwrap().cmp(&b.path.symlink_metadata().unwrap().created().unwrap()))
}

fn modified_sort(dir: &mut Vec<File>) {
  dir.sort_by(|a, b| a.path.symlink_metadata().unwrap().modified().unwrap().cmp(&b.path.symlink_metadata().unwrap().modified().unwrap()))
}

fn size_sort(dir: &mut Vec<File>) {
  dir.sort_by(|a, b| a.path.symlink_metadata().unwrap().size().cmp(&b.path.symlink_metadata().unwrap().size()))
}

impl std::fmt::Display for File {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut res = String::new();
    for (i, v) in self.file_type.iter().enumerate() {
      if i == 0 {
        res = v.get_text_traits_for_type(self.path.file_name().unwrap().to_str().unwrap(), &self.path);
        res = format!("{}{}", v.get_color_for_type(), res);
      } else {
        res = v.get_text_traits_for_type(&res, &self.path);
        res = format!("{}{}", v.get_color_for_type(), res);
      }
    }
    write!(f, "{}", res )
  }
}

impl std::fmt::Display for Directory {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    Ok(
      for i in self.paths.iter() {
        write!(f, "{}\n", i)?;
      }
    )
  }
}

fn main() {
  let mut dir = Directory::new(input::Cli::from_args().dir).unwrap();
  dir.sort();
  println!("{}", dir)
}

#[cfg(test)]
mod tests;
