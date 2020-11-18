use structopt::StructOpt;

/// The ls replacement you never knew you needed
#[derive(StructOpt)]
pub struct Cli {
  #[structopt(parse(from_os_str), default_value=".")]
  pub dir: std::path::PathBuf,
}
