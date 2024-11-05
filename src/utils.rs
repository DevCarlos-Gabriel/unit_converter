// Reexportando a função input_user do módulo input_user
pub use crate::input_user::input_user;
// Reexportando a função temperature_converter do módulo temperature_converters
pub use crate::unit_converter::*;
// Reexportando as constantes que temos no módulos contants.rs
pub use crate::constants;
// Reexportando a biblioteca padrão para usar com o esse módulo utils
pub use std::io::{self,Write};

//pub use crate::terminal_utils;
