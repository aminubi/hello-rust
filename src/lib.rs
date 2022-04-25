pub fn hello(){
	println!("Hello, world!");
}

pub fn compute(x: i32, y: i32) -> i32 {
	x + y
}

// create vector of i32
pub fn create_vector() -> Vec<i32> {
	vec![1, 2, 3, 4, 5]
}

// create vector of String
pub fn create_vector_string() -> Vec<String> {
	vec![String::from("Hello"), String::from("World")]
}