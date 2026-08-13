//@no-rustfix

#![warn(clippy::inlined_format_args)]
#![allow(named_arguments_used_positionally)]

fn main() {
    let local_i32 = 1;

    // positions pointing into the named arguments have no fix
    println!("{local_i32} {0}", name = 1);
    //~^ inlined_format_args
    println!("{local_i32} {}", name = 1);
    //~^ inlined_format_args
}
