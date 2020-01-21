#[derive(Default, PartialEq, Clone)]
pub struct Program {
	functions: Vec<crate::function::Function>,
	results: Vec<crate::result::RoResult>
}