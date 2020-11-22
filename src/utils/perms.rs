use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::os::unix::fs::{PermissionsExt};

pub fn perms(file: std::path::PathBuf) -> String {
  let mode = file.symlink_metadata().unwrap().permissions().mode() as u16;
  let user = masking(mode, S_IRUSR as u16, S_IWUSR as u16, S_IXUSR);
  let group = masking(mode, S_IRGRP as u16, S_IWGRP as u16, S_IXGRP);
  let other = masking(mode, S_IROTH as u16, S_IWOTH as u16, S_IXOTH);
  let f = crate::PathType::new(&file).unwrap()[0].get_letter_for_type();
  [f, user, group, other].join("")
}

fn masking(mode: u16, read: u16, write: u16, execute: u16) -> String {
  match (mode & read, mode & write, mode & execute) {
    (0, 0, 0) => "---",
    (_, 0, 0) => "r--",
    (0, _, 0) => "-w-",
    (0, 0, _) => "--x",
    (_, 0, _) => "r-x",
    (_, _, 0) => "rw-",
    (0, _, _) => "-wx",
    (_, _, _) => "rwx",
  }.to_string()
}
