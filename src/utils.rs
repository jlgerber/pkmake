use std::ops::{Bound, RangeBounds};

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

use anyhow::anyhow;
use anyhow::Error as AnyError;
use shellfn::shell;

#[shell]
pub fn exec_in_shell(fn_str: &str) -> Result<String, AnyError> {
    r#"$FN_STR"#
}

pub use subprocess::ExitStatus;
use subprocess::{Exec, Redirection};
pub fn exec_cmd(cmds: Vec<String>) -> Result<ExitStatus, AnyError> {
    if cmds.len() < 1 {
        return Err(anyhow!("must pass at least one command to exec_cmd"));
    }
    let mut phandle = Exec::shell(cmds[0].as_str())
        //.stdout(Redirection::Pipe)
        // .stderr(Redirection::Merge)
        .popen()?;
    for cmd in &cmds[1..] {
        let result = phandle.communicate(Some(cmd.as_str()))?;
        if result.0.is_some() {
            println!("{}", result.0.unwrap());
        }
        if result.1.is_some() {
            println!("{}", result.1.unwrap());
        }
    }
    let exit_status = phandle.wait()?;
    Ok(exit_status)
}
