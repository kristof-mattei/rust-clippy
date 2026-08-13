//@aux-build:proc_macros.rs

#![warn(clippy::inlined_format_args)]
#![allow(named_arguments_used_positionally, unused)]
#![allow(clippy::assertions_on_constants, clippy::eq_op, clippy::print_literal)]

extern crate proc_macros;
use proc_macros::with_span;

use std::fmt::Write;

macro_rules! my_println {
   ($($args:tt),*) => {{
        println!($($args),*)
    }};
}

fn tester(fn_arg: i32) {
    let local_i32 = 1;
    let local_f64 = 2.0;
    let width = 4;
    let prec = 2;

    println!("val='{local_i32}'");
    //~^ inlined_format_args
    println!("{local_i32}");
    //~^ inlined_format_args
    println!("{fn_arg}");
    //~^ inlined_format_args
    println!("{local_i32:?}");
    //~^ inlined_format_args
    println!("{local_i32:#?}");
    //~^ inlined_format_args
    println!("{local_i32:4}");
    //~^ inlined_format_args
    println!("{local_i32:04}");
    //~^ inlined_format_args
    println!("{local_i32:<3}");
    //~^ inlined_format_args
    println!("{local_i32:#010x}");
    //~^ inlined_format_args
    println!("{local_f64:.1}");
    //~^ inlined_format_args
    println!("{local_i32} {local_f64}");
    //~^ inlined_format_args
    println!("{local_i32} {local_i32}");
    //~^ inlined_format_args
    println!("{local_f64} {local_i32} {local_f64} {local_i32}");
    //~^ inlined_format_args

    // captured width and precision
    println!("{local_i32:width$}");
    //~^ inlined_format_args
    println!("{local_i32:>width$}");
    //~^ inlined_format_args
    println!("{local_f64:.prec$}");
    //~^ inlined_format_args
    println!("{local_f64:width$.prec$}");
    //~^ inlined_format_args
    println!("{width:width$}");
    //~^ inlined_format_args
    println!("{:width$}", local_i32);
    //~^ inlined_format_args
    println!("{:.prec$}", local_f64);
    //~^ inlined_format_args

    // mixed with explicit arguments
    println!("{} {local_i32}", 42);
    //~^ inlined_format_args
    println!("{local_i32} {}", 42);
    //~^ inlined_format_args
    println!("{0} {local_i32}", 42);
    //~^ inlined_format_args
    println!("{local_i32:.*}", 2);
    //~^ inlined_format_args
    println!("{local_i32:width$}", width = 4);
    //~^ inlined_format_args
    println!("{name} {local_i32}", name = fn_arg);
    //~^ inlined_format_args

    let mut s = String::new();
    write!(s, "{local_i32}");
    //~^ inlined_format_args
    writeln!(s, "{local_i32}");
    //~^ inlined_format_args
    let _ = format!("{local_i32}");
    //~^ inlined_format_args
    eprintln!("{local_i32}");
    //~^ inlined_format_args
    assert!(true, "{local_i32}");
    //~^ inlined_format_args
    assert_eq!(1, 1, "{local_i32}");
    //~^ inlined_format_args
    if false {
        panic!("{local_i32}");
        //~^ inlined_format_args
    }

    // keywords are captured without `r#`, the suggested argument needs it
    let r#type = 1;
    println!("{type}");
    //~^ inlined_format_args

    // no captures, no lint
    println!("{}", local_i32);
    println!("{0}", local_i32);
    println!("{v}", v = local_i32);
    println!("{{local_i32}}");
    println!("42");

    // the format string is not written at the call site, no lint
    my_println!("{local_i32}");
    with_span!(span println!("{local_i32}"));
}

#[derive(Debug)]
struct S;

impl S {
    // `self` cannot be a raw identifier, the suggested argument stays bare
    fn debug(&self) {
        println!("{self:?}");
        //~^ inlined_format_args
    }
}

fn main() {
    tester(42);
}
