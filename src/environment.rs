use std::ffi::OsString;

pub trait Environment {
    fn var_os(&self, name: &str) -> Option<OsString>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SystemEnvironment;

impl Environment for SystemEnvironment {
    fn var_os(&self, name: &str) -> Option<OsString> {
        std::env::var_os(name)
    }
}
