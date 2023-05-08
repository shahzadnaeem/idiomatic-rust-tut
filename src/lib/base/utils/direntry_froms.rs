use std::fs::DirEntry;

use crate::base::prelude::*;

impl TryFrom<NewT<&DirEntry>> for String {
    type Error = Error;

    fn try_from(value: NewT<&DirEntry>) -> Result<String> {
        value
            .0
            .path()
            .to_str()
            .map(String::from)
            .ok_or_else(|| Error::Todo(fmt!("Invalid path {:?}", value.0)))
    }
}
