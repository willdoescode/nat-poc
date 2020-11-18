use structopt::StructOpt;

/// The ls replacement you never knew you needed
#[derive(StructOpt)]
pub struct Cli {
  /// Give me a directory
  #[structopt(parse(from_os_str), default_value=".")]
  pub dir: std::path::PathBuf,

  #[structopt(short = "n", long = "name")]
  pub name: bool,

  #[structopt(short = "c", long = "created")]
  pub created: bool,

  #[structopt(short = "m", long = "modified")]
  pub modified: bool,

  #[structopt(short = "s", long = "size")]
  pub size: bool,
}
