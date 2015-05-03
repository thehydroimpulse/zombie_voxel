#![feature(custom_derive, plugin, custom_attribute)]
#![plugin(gfx_macros)]

extern crate gfx;
extern crate cgmath;

pub mod block;
pub mod chunk;
pub mod shaders;
pub mod renderer;
