use structopt::StructOpt;

/// The ls replacement you never knew you needed
#[derive(StructOpt)]
pub struct Cli {
  /// Give me a directory
  #[structopt(parse(from_os_str), default_value=".")]
  pub dir: std::path::PathBuf,
  
  /// Sorts files by name
  #[structopt(short = "n", long = "name")]
  pub name: bool,
  
  /// Sorts files by the date created
  #[structopt(short = "c", long = "created")]
  pub created: bool,

  /// Sorts files by the date modified
  #[structopt(short = "m", long = "modified")]
  pub modified: bool,

  /// Sorts files by file size
  #[structopt(short = "s", long = "size")]
  pub size: bool,
  
  /// Groups directorys before files
  #[structopt(short = "g", long = "gdf")]
  pub gdf: bool,
}
