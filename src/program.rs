/*
 * Copyright (c) 2019 Mike White
 *
 * @Script: program.rs
 * @Author: Mike White
 * @Email: botahamec@outlook.com
 * @Create At: 2019-11-14 18:22:57
 * @Last Modified By: Mike White
 * @Last Modified At: 2019-11-14 19:37:34
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
	pub main_result: RoResult
}

impl Default for RoProgram {
	fn default() -> Self {
		RoProgram {
			results: HashMap::new(),
			functions: HashMap::new(),
			structs: HashMap::new(),
			main_result: RoResult::new(
					String::from("main"),
					HashMap::new(),
					RoType::None,
					ResultCall::Result,
					true,
					false,
					true
			)
		}
	}
}

impl RoProgram {
	fn new() -> Self {Default::default()}
}

fn test() {
	let result = RoResult::new(String::from("main"),
					HashMap::new(),
					RoType::None,
					ResultCall::Result,
					true,
					false,
					true);
	println!("{}", result.name);
}