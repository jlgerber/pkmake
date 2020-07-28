//! Utils
//!
//! Contains the StringUtils trait with is used to add a ```substring``` and
//! ```slice``` method to ```str``` allowing us to get substrings 
//! based on chars without additional allocations
//!
//! Also provides the ```exec_cmd```, which is responsible for executing a 
//! vec of commands in a subprocess, given a working directory, returning 
//! the results. The command does not capture stderr or stdout, allowing them
//! to be viewed by the end user in realtime.

// external crate imports
use anyhow::anyhow;
use anyhow::Error as AnyError;
use std::ops::{Bound, RangeBounds};
use subprocess::Exec;

// public external crate imports 
pub use subprocess::ExitStatus;

pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else {
                break;
            }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else {
                break;
            }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}


// we are using subprocess instead
// use shellfn::shell;

// #[shell]
// pub fn exec_in_shell(fn_str: &str) -> Result<String, AnyError> {
//     r#"$FN_STR"#
// }



pub fn exec_cmd<I>(cmds: &str, cwd: I) -> Result<ExitStatus, AnyError>
where
    I: AsRef<std::path::Path>,
{
    if cmds.len() < 1 {
        return Err(anyhow!("must pass at least one command to exec_cmd"));
    }

    let exit_status = Exec::shell(cmds).cwd(cwd.as_ref()).join()?;
    Ok(exit_status)
}
