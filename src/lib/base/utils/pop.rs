use crate::base::prelude::*;

pub fn pop(n: i32) -> Result<i32> {
    if n < 0 {
        Err(Error::BadRequest(n, "-ve is BAD!".to_string()))
    } else {
        Ok(n)
    }
}
