/*
 * Copyright (c) 2019 Mike White
 *
 * @Script: program.rs
 * @Author: Mike White
 * @Email: botahamec@outlook.com
 * @Create At: 2019-11-14 18:22:57
 * @Last Modified By: Mike White
 * @Last Modified At: 2019-11-14 20:05:08
 * @Description: Container for all the required objects of a Ro program
 */

use std::collections::HashMap;
use crate::result::RoResult;
use crate::result::RoFunction;
use crate::rotype::RoStruct;
use crate::rotype::RoType;
use crate::result::ResultCall;

pub struct RoProgram {
	pub results: HashMap<String, RoResult>,
	pub functions: HashMap<String, RoFunction>,
	pub structs: HashMap<String, RoStruct>,
	pub main_fn: RoResult
}


impl Default for RoProgram {
	fn default() -> Self {
		RoProgram {
			results: HashMap::new(),
			functions: HashMap::new(),
			structs: HashMap::new(),
			main_fn: RoResult{
					name: String::from("main"),
					parameters: HashMap::new(),
					return_type: RoType::None,
					call: ResultCall::Result,
					optimized: true,
					skippable: false,
					compilable: true
			}
		}
	}
}

impl RoProgram {

	pub fn new() -> Self {Default::default()}

	fn add_result(&mut self, result: RoResult) {
		self.results.insert(result.name.clone(), result);
	}

	fn add_fn(&mut self, function: RoFunction) {
		self.functions.insert(function.name.clone(), function);
	}

	fn add_struct(&mut self, ro_struct: RoStruct) {
		self.structs.insert(ro_struct.name.clone(), ro_struct);
	}

	fn set_main_fn(&mut self, result: RoResult) {
		self.main_fn = result;
	}
}