// temp for testing
use autocxx::prelude::*;
fn main() {
    chirp_sys::bridge::ir_builder_new();
    //C++ Heap: within_unique_ptr()
    //Rust Heap: within_box()
    //Rust Stack: just new()
    let mut builder = chirp_sys::ir_builder::taichi::lang::IRBuilder::new().within_box();
    // let mut sum = builder.as_mut().create_local_var();
    // let mut sum_ptr = &sum;
    // unsafe {
    //     builder.as_mut().create_local_store(*sum_ptr, builder.as_mut().get_float32(0.2));
    // }
}