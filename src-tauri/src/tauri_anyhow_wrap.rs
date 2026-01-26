#[macro_export]
macro_rules! tauri_bail {
    ($($arg:tt)*) => {{
        return Err(anyhow::anyhow!($($arg)*).into());
    }};
}

#[macro_export]
macro_rules! tauri_ensure {
    ($cond:expr $(,)?) => {
        if !$cond {
            tauri_bail!("condition `{}` not satisfied", stringify!($cond));
        }
    };
    ($cond:expr, $expr:expr $(,)?) => {
        if !$cond {
            tauri_bail!($expr);
        }
    };
    ($cond:expr, $msg:literal $(, $arg:expr)* $(,)?) => {
        if !$cond {
            tauri_bail!($msg, $($arg,)*);
        }
    };
}
