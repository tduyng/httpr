#[macro_export]
macro_rules! handler {
    (|$ctx:ident| {$($b:tt)+}) => {{
        let closure = |$ctx: &mut $crate::Context| -> $crate::Handler {
            Box::pin(async move {
                $($b)+;
                Ok(())
            })
        };
        closure
    }};
}
