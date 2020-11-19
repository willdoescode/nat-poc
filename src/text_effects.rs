pub fn bold<'a>() -> (&'a str, &'a str) {
  ("\x1B[1m", "\x1B[0m")
}

pub fn underline<'a>() -> (&'a str, &'a str) {
  ("\x1B[4m", "\x1B[0m")
}

pub fn dimmed<'a>() -> (&'a str, &'a str) {
  ("\x1B[2m", "\x1B[0m")
}

pub fn italic<'a>() -> (&'a str, &'a str) {
  ("\x1B[3m", "\x1B[0m")
}

pub fn blink<'a>() -> (&'a str, &'a str) {
  ("\x1B[5m", "\x1B[0m")
}

pub fn reverse<'a>() -> (&'a str, &'a str) {
  ("\x1B[7m", "\x1B[0m")
}

pub fn hidden<'a>() -> (&'a str, &'a str) {
  ("\x1B[8m", "\x1B[0m")
}

pub fn stricken<'a>() -> (&'a str, &'a str) {
  ("\x1B[9m", "\x1B[0m")
}
