#[macro_export]
macro_rules! felt_vec {
    ($($a:expr),*) => {
        vec![$(starknet_types_core::felt::Felt::from($a)),*]
    };
}

#[macro_export]
macro_rules! arg_value {
    ($a:expr) => {
        cairo_lang_runner::Arg::Value(starknet_types_core::felt::Felt::from($a))
    };
}

#[macro_export]
macro_rules! arg_value_vec {
    ($($a:expr),*) => {
        vec![$(cairo_lang_runner::Arg::Value(starknet_types_core::felt::Felt::from($a))),*]
    };
}

#[macro_export]
macro_rules! arg_array {
    ($($a:expr),*) => {
        cairo_lang_runner::Arg::Array(felt_vec![$($a),*])
    };
}
