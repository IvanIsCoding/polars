use std::sync::RwLock;

type WarningFunction = fn(&str, PolarsWarning);
static WARNING_FUNCTION: RwLock<Option<WarningFunction>> = RwLock::new(None);

/// Set the function that will be called by the `polars_warn!` macro.
/// You can use this to set logging in polars.
///
/// # Safety
/// The caller must ensure there is no other thread accessing this function
/// or calling `polars_warn!`.
pub fn set_warning_function(function: WarningFunction) {
    let mut w = WARNING_FUNCTION.write().unwrap();
    *w = Some(function)
}
#[derive(Debug)]
pub enum PolarsWarning {
    UserWarning,
    CategoricalRemappingWarning,
    MapWithoutReturnDtypeWarning,
}

fn eprintln(fmt: &str, warning: PolarsWarning) {
    eprintln!("{:?}: {}", warning, fmt);
}

pub fn get_warning_function() -> WarningFunction {
    WARNING_FUNCTION.read().unwrap().unwrap_or(eprintln)
}
#[macro_export]
macro_rules! polars_warn {
    ($variant:ident, $fmt:literal $(, $arg:tt)*) => {
        {{
        let func = $crate::get_warning_function();
        let warn = $crate::PolarsWarning::$variant;
        func(format!($fmt, $($arg)*).as_ref(), warn)
        }}
    };
    ($fmt:literal, $($arg:tt)+) => {
        {{
        let func = $crate::get_warning_function();
        func(format!($fmt, $($arg)+).as_ref(), $crate::PolarsWarning::UserWarning)
        }}
    };
    ($($arg:tt)+) => {
        polars_warn!("{}", $($arg)+);
    };
}
