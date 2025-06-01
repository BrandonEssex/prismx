#[macro_export]
macro_rules! declare_toggle {
    ($name:ident) => {
        // Marker macro used by CI to assert toggle usage
        #[allow(dead_code)]
        const _: () = {
            let _ = stringify!($name);
        };
    };
}
