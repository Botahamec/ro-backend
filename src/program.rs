#[derive(Default, PartialEq, Clone)]
pub struct Program {
	pub functions: Vec<crate::function::Function>,
	pub results: Vec<crate::result::RoResult>
}