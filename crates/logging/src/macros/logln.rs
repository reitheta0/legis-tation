#[macro_export]
macro_rules! logln {
  ($chr: literal, $($arg:tt)*) => {
    quick_log(&format!($($arg)*), $chr, false);
    println!($($arg)*);
  };
}
