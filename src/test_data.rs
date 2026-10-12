
#[cfg(test)]
pub mod test_data {
    use byte_serializer::serialize;
    #[derive(Debug)]
    pub struct Person {
        pub name:String,
        pub age:u64,
        pub height:f64,
        pub male: bool,
    }

impl Person {
    pub fn new(name:String,age:u64,height:f64,male:bool) -> Person {
        Person {name,age,height,male}
    }
}









}
