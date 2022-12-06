macro_rules! res_or_crash_with {
    ($res:expr, $err_msg:expr) => {
        match $res {
            Ok(v) => v,
            Err(e) => {
                panic!(format!("{} Error: {:?}", $err_msg, e));
            }
        }
    };
}

macro_rules! val_or_crash_with {
    ($res:expr, $err_msg:expr) => {
        match $res {
            Some(v) => v,
            None => {
                panic!(format!("Expected value not found: {}", $err_msg));
            }
        }
    };
}
