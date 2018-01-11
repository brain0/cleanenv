use std::env;
use std::process;
use std::os::unix::prelude::*;

static PULSE_PROP_PREFIX: &'static str = "PULSE_PROP_";

fn main() {
    for (var, _) in env::vars_os() {
        if let Some(s) = var.to_str() {
            if s.starts_with(PULSE_PROP_PREFIX) {
                env::remove_var(s);
            }
        }
    }
    let mut args = env::args_os();
    args.next();
    let mut cmd = process::Command::new(args.next().unwrap());
    while let Some(arg) = args.next() {
        cmd.arg(arg);
    }
    let err = cmd.exec();
    panic!("exec() failed: {:?}", err);
}
