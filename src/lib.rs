/*
 * Copyright (c) 2019 Mike White
 *
 * @Script: lib.rs
 * @Author: Mike White
 * @Email: botahamec@outlook.com
 * @Create At: 2019-11-09 20:46:46
 * @Last Modified By: Mike White
 * @Last Modified At: 2019-11-14 19:07:31
 * @Description: The library for Ro, a result oriented programming language
 */

#![allow(dead_code)]

pub mod call;
pub mod function;
pub mod parameter;
pub mod program;
pub mod result;

pub use call::Call;
pub use call::CallType;
pub use function::Function;
pub use parameter::Parameter;
pub use program::Program;
pub use result::RoResult;
