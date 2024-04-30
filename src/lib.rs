#[macro_export]

/// A macro to unwrap a Result if ok, otherwise log and do control flow expression
/// 
/// usage: 
/// ```
/// let x = result_unwrap_or!(result, error!("message"), return;);
/// let y = result_unwrap_or!(result, error!("message"), return 3;);
/// let z = result_unwrap_or!(result, error!("message, {}", something_else), 3);
/// ```
macro_rules! result_unwrap_or {
    ($expr:expr, error!($($arg:tt)+), $($ret:tt)+) => {
        result_unwrap_or_inner!($expr, error, x($($ret)+), $($arg)+)
    };
    ($expr:expr, warn!($($arg:tt)+), $($ret:tt)+) => {
        result_unwrap_or_inner!($expr, warn, x($($ret)+), $($arg)+)
    };
    ($expr:expr, info!($($arg:tt)+), $($ret:tt)+) => {
        result_unwrap_or_inner!($expr, info, x($($ret)+), $($arg)+)
    };
    ($expr:expr, debug!($($arg:tt)+), $($ret:tt)+) => {
        result_unwrap_or_inner!($expr, debug, x($($ret)+), $($arg)+)
    };
    ($expr:expr, trace!($($arg:tt)+), $($ret:tt)+) => {
        result_unwrap_or_inner!($expr, trace, x($($ret)+), $($arg)+)
    };
}

macro_rules! result_unwrap_or_inner {
    ($expr:expr, $level:ident, x($($ret:tt)+), $($arg:tt)+) => {
        match $expr {
            Ok(ok) => ok,
            Err(err) => {
                tracing::$level!("{}", format!("{:?}\n{:?}", format_args!($($arg)+), err));
                $($ret)+
            }
        }
    };
}


/// A macro to unwrap an Option if Some, otherwise log and do control flow expression
/// 
/// usage: 
/// ```
/// let x = unwrap_or_opt!(opt, error!("message"), return;);
/// let y = unwrap_or_opt!(opt, error!("message"), return 3;);
/// let z = unwrap_or_opt!(opt, error!("message, {}", something_else), 3);
/// ```
macro_rules! unwrap_or_opt {
    ($expr:expr, error!($($arg:tt)+), $($ret:tt)+) => {
        unwrap_or_opt_inner!($expr, error, x($($ret)+), $($arg)+)
    };
    ($expr:expr, warn!($($arg:tt)+), $($ret:tt)+) => {
        unwrap_or_opt_inner!($expr, warn, x($($ret)+), $($arg)+)
    };
    ($expr:expr, info!($($arg:tt)+), $($ret:tt)+) => {
        unwrap_or_opt_inner!($expr, info, x($($ret)+), $($arg)+)
    };
    ($expr:expr, debug!($($arg:tt)+), $($ret:tt)+) => {
        unwrap_or_opt_inner!($expr, debug, x($($ret)+), $($arg)+)
    };
    ($expr:expr, trace!($($arg:tt)+), $($ret:tt)+) => {
        unwrap_or_opt_inner!($expr, trace, x($($ret)+), $($arg)+)
    };
}

macro_rules! unwrap_or_opt_inner {
    ($expr:expr, $level:ident, x($($ret:tt)+), $($arg:tt)+) => {
        match $expr {
            Some(val) => val,
            None => {
                tracing::$level!($($arg)+);
                $($ret)+
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result_fn() -> Result<u32, String> {
        Err("This is an error".to_string())
    }

    fn option_fn() -> Option<u32> {
        None
    }

    #[test]
    fn unwrap_or_res() {
        tracing_subscriber::fmt().init();
        let result = result_fn();
        let _x: u32 = result_unwrap_or!(result, error!("message {}", 1), return;);
        let _x: u32 = result_unwrap_or!(result, trace!("message"), 1);
    }

    #[test]
    fn unwrap_or_opt() {
        let opt = option_fn();
        let _x: u32 = unwrap_or_opt!(opt, error!("message {}", 1), return;);
        let _x: u32 = unwrap_or_opt!(opt, trace!("message"), 1);
    }
}
