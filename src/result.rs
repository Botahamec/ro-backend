#[derive(Default, PartialEq, Clone)]
pub struct RoResult {
	pub name: String,
	pub parameters: Vec<crate::parameter::Parameter>,
	pub return_type: String,
	pub functions: Vec<String>,
}