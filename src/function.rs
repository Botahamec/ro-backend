#[derive(Default, PartialEq, Clone)]
pub struct Function {
	name: String,
	parameters: Vec<crate::parameter::Parameter>,
	return_type: String,
	calls: Vec<crate::call::Call>
}