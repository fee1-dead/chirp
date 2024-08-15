import re
import os

def generate_stmts(rust_output_file):
    statements_fn = os.path.join(os.path.dirname(__file__), "../chirp-sys/taichi/taichi/ir/statements.h")
    with open(statements_fn, 'r') as f:
        content = f.read()

    class_names = re.findall(r'class\s+(\w+Stmt)\s*:', content)

    with open(rust_output_file, 'w') as f:
        f.write("""
use chirp_sys::taichi::lang;

pub trait Stmt {
    fn cast(&mut self) -> *mut lang::Stmt;
}
""")
        for class_name in class_names:
            rust_struct = f"""
pub struct {class_name} {{
    inner: *mut lang::{class_name},
}}

impl {class_name} {{
    pub fn new(inner: *mut lang::{class_name}) -> Self {{
        {class_name} {{ inner }}
    }}
    pub fn into_raw(&self) -> &*mut lang::{class_name} {{
        &self.inner
    }}
}}

impl Stmt for {class_name} {{
    fn cast(&mut self) -> *mut lang::Stmt {{
        self.inner.cast()
    }}
}}
"""
            f.write(rust_struct)
    
if __name__ == "__main__":
    generate_stmts('../chirp-wrapper/src/ir/stmts.rs')