#[derive(Debug)]
pub struct DemoStruct {
    pub f1: i64,
    pub f2: u64,
    pub f3: Option<i32>, 
    pub f4: f32,
    pub f5: f64,
}


pub fn return_demo_struct () -> DemoStruct {
    DemoStruct { f1: -1, f2: 100, f3: Some(-99), f4: 0.31, f5: 0.0000048 }
}

pub fn return_demo_struct_result() -> Result<DemoStruct, String> {
    Ok(DemoStruct{f1: -1, f2: 100, f3: Some(-99), f4: 0.31, f5: 0.0000048 })
}