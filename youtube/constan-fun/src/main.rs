pub mod const_fn_with_const_generic;
mod constants_compile_time;
pub mod normal_constant_fn;
pub mod const_str_variant;

fn main() {
    const_fn_with_const_generic::test();
}
