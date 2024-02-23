use rust_rvo_test::{return_demo_struct, return_demo_struct_result};


fn main() {
    let demo_struct = return_demo_struct();
    println!("{:?}", demo_struct);

    let demo_struct_result = return_demo_struct_result();
    
    println!("{:?}", demo_struct_result.unwrap());
}