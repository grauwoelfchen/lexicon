#![feature(slice_patterns)]

extern crate lexicon;

use std::env;

use lexicon::workspace;

fn main() {
  let mut args = env::args();
  let path;
  if args.len() == 2 {
    // 1 arg
    path = args.nth(1).unwrap();
  } else {
    panic!("run: lexicon <file>");
  }

  // just display package attributes
  let ws = workspace::load(&path);
  if let Some(packages) = ws.packages {
    for package in packages {
      println!("{}", package);
    }
  }
}
