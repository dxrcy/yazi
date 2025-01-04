use yazi_macro::render;
use yazi_shared::event::CmdCow;

use crate::input::{Input, InputMode, op::InputOp};

impl Input {
	#[yazi_codegen::command]
	pub fn replace(&mut self, _: CmdCow) {
		let snap = self.snap_mut();
		if snap.mode == InputMode::Normal {
			snap.op = InputOp::None;
			snap.mode = InputMode::Replace;
			render!();
		}
	}

	pub fn replace_str(&mut self, s: &str) {
		let snap = self.snaps.current_mut();

		let start = snap.idx(snap.cursor).unwrap();
		let end = snap.idx(snap.cursor + 1).unwrap();
		snap.value.replace_range(start..end, s);

		self.flush_value();
		render!();
	}
}
