pub use rutenspitz_macro::arbitrary_stateful_operations;

lazy_static::lazy_static! {
    pub static ref NON_DEBUG_PANIC_HOOK: () = {
        std::panic::set_hook(Box::new(|panic_info| {
            if let Some(outpanic) = panic_info.payload()
                .downcast_ref::<crate::OutcomePanic>()
            {
                eprintln!("{}", outpanic.0);
                std::process::abort();
            }
        }))
    };
}

#[macro_export]
macro_rules! panic {
    ($($arg:tt)*) => { std::panic!(rutenspitz::OutcomePanic(format!($($arg)*))) };
}

pub struct OutcomePanic(pub String);

pub use lazy_static;

pub mod derive {
    pub use arbitrary::Arbitrary;
    pub use strum_macros::IntoStaticStr;
}
