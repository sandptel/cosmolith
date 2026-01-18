use serde_json::Value;
use treediff::diff;

pub fn print_set(ns: &str, key: &str, new_val: &Value) {
	println!("[{ns}] {key} set:");
	print_value_lines("  ", new_val);
}

pub fn print_diffs(ns: &str, key: &str, old: &Value, new_val: &Value) {
	let mut delegate = DiffPrinter::new(ns, key);
	diff(old, new_val, &mut delegate);
	delegate.flush();
}

struct DiffPrinter<'a> {
	ns: &'a str,
	root: &'a str,
	stack: Vec<String>,
	changes: Vec<String>,
}

impl<'a> DiffPrinter<'a> {
	fn new(ns: &'a str, root: &'a str) -> Self {
		Self {
			ns,
			root,
			stack: Vec::new(),
			changes: Vec::new(),
		}
	}

	fn flush(self) {
		if self.changes.is_empty() {
			return;
		}
		println!("[{ns}] {root} changed:", ns = self.ns, root = self.root);
		for line in self.changes {
			println!("  {line}");
		}
	}

	fn path_with<K: ToString>(&self, key: Option<&K>) -> String {
		let mut parts = Vec::with_capacity(1 + self.stack.len() + key.map(|_| 1).unwrap_or(0));
		parts.push(self.root.to_string());
		parts.extend(self.stack.iter().cloned());
		if let Some(k) = key {
			parts.push(k.to_string());
		}
		parts.join(".")
	}
}

impl<'a, K> treediff::Delegate<'a, K, Value> for DiffPrinter<'_>
where
	K: ToString + Clone,
{
	fn push(&mut self, k: &K) {
		self.stack.push(k.to_string());
	}

	fn pop(&mut self) {
		self.stack.pop();
	}

	fn added<'b>(&mut self, k: &'b K, v: &'a Value) {
		let path = self.path_with(Some(k));
		self.changes.push(format!("+ {path}: {v:?}"));
	}

	fn removed<'b>(&mut self, k: &'b K, v: &'a Value) {
		let path = self.path_with(Some(k));
		self.changes.push(format!("- {path}: {v:?}"));
	}

	fn modified(&mut self, old: &'a Value, new: &'a Value) {
		let path = self.path_with::<String>(None);
		self.changes
			.push(format!("~ {path}: {old:?} -> {new:?}"));
	}
}

fn print_value_lines(indent: &str, val: &Value) {
	match val {
		Value::Object(map) => {
			println!("{indent}{{");
			for (k, v) in map {
				print!("{indent}  {k}: ");
				print_value_inline(&format!("{indent}  "), v);
			}
			println!("{indent}}}");
		}
		_ => println!("{indent}{val:?}"),
	}
}

fn print_value_inline(indent: &str, val: &Value) {
	match val {
		Value::Object(map) => {
			println!("{{");
			for (k, v) in map {
				print!("{indent}  {k}: ");
				print_value_inline(&format!("{indent}  "), v);
			}
			println!("{indent}}}");
		}
		_ => println!("{val:?}"),
	}
}
