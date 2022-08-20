//  LIB.rs
//    by Lut99
// 
//  Created:
//    26 Mar 2022, 10:31:02
//  Last edited:
//    06 Aug 2022, 16:23:27
//  Auto updated?
//    Yes
// 
//  Description:
//!   The `rust-ecs` crate implements a (simple) Entity Component System in
//!   Rust. It is designed to run smoothly and integrate nicely with the Rust
//!   idioms.
//! 
//!   This crate is already being used in the [Game-Rust](https://github.com/Lut99/Game-Rust)
//!   project.
// 

// Declare the modules
pub mod spec;
pub mod list;
pub mod system;


// Bring some components into the general package namespace (possibly by aliasing them)
pub use spec::{Component, Entity};
pub use list::ComponentList;
pub use system::Ecs;


// Define some useful macros
/// Downcasts a generic ComponentListBase to a ComponentList<T>
#[macro_export]
macro_rules! to_component_list {
    ($list:expr,$ctype:tt) => {
        {
            let name = $list.type_name();
            $list.as_any().downcast_ref::<ComponentList<$ctype>>().expect(&format!("Could not downcast ComponentList<{}> to ComponentList<{}>", name, ComponentList::<$ctype>::type_name()))
        }
    };
}

/// Downcasts a generic ComponentListBase to a ComponentList<T>
#[macro_export]
macro_rules! to_component_list_mut {
    ($list:expr,$ctype:tt) => {
        {
            let name = $list.type_name();
            $list.as_any_mut().downcast_mut::<ComponentList<$ctype>>().expect(&format!("Could not downcast ComponentList<{}> to ComponentList<{}>", name, ComponentList::<$ctype>::type_name()))
        }
    };
}


// Define some crate-local macros
/// Performs a `log`-crate `debug`, but only if that feature is defined
#[cfg(feature = "log")]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => {
        log::debug!($target, $($arg)+)
    };

    ($($arg:tt)+) => {
        log::debug!($($arg)+)
    };
}
#[cfg(not(feature = "log"))]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => { () };

    ($($arg:tt)+) => { () };
}
pub(crate) use debug;
