/*
 * Copyright (c) 2019 Mike White
 *
 * @Script: result.rs
 * @Author: Mike White
 * @Email: botahamec@outlook.com
 * @Create At: 2019-11-09 20:48:47
 * @Last Modified By: Mike White
 * @Last Modified At: 2019-11-14 19:37:15
 * @Description: Contains the result type used by Ro
 */

use crate::rotype::RoType;
use crate::rule::FunctionRule;
use std::collections::HashMap;

pub enum ResultCall {
	Operation(RoOperation),
	Result
}

pub struct RoResult {
	pub name: String,
	pub parameters: HashMap<String, RoType>,
	pub return_type: RoType,
	pub call: ResultCall,
	pub optimized: bool,
	pub skippable: bool,
	pub compilable: bool
}

pub struct RoFunction {
	pub name: String,
	pub result: String, // the name of the result attached to the function
	pub execution: RoResult, // the result that gets run by the function
	pub rules: Vec<FunctionRule>
}

pub struct RoOperation {
	operation: usize,
	parameters: Vec<usize>
}

impl RoResult {
	pub fn new (
			name: String,
			parameters: HashMap<String, RoType>,
			return_type: RoType,
			call: ResultCall,
			optimized: bool,
			skippable: bool,
			compilable: bool
			) -> Self {
		RoResult {
			name: name,
			parameters: parameters,
			return_type: return_type,
			call: call,
			optimized: optimized,
			skippable: skippable,
			compilable: compilable
		}
	}
}