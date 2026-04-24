/*#[macro_export]
macro_rules! if_let_grab {
  ($expr_grab:expr, $ident_grab:ident, $grabbed:expr) => {
    if let Some($ident_grab) = $grabbed {
      logln!('t', "Ran Successfully: Here's the name: {}!", $expr_grab);
    } else {
      logln!('i', "Ran Successfully: There's no name for this one.");
    }
  };
}

#[macro_export]
macro_rules! grab {
  ($node:ident, $grab:ident, $get_grab:expr) => {
    let literal_grab = stringify!($grab);
    if_let_grab!(literal_grab, $grab, $node.$get_grab());
  };
}*/
