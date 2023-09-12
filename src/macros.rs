#[macro_export]
macro_rules! dll_function {
    (fn $func:ident$args:tt $block:block) => {
        #[no_mangle]
        pub extern "C" fn $func$args $block
    };
}