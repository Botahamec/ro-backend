#[derive(Default, PartialEq, Clone)]
pub struct Function {
	pub name: String,
	pub parameters: Vec<crate::parameter::Parameter>,
	pub return_type: String,
	pub calls: Vec<crate::call::Call>
}