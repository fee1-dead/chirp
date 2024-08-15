import re
import os

def generate_stmts(rust_output_file):
    statements_fn = os.path.join(os.path.dirname(__file__), "../chirp-sys/taichi/taichi/ir/statements.h")
    with open(statements_fn, 'r') as f:
        content = f.read()

    class_names = re.findall(r'class\s+(\w+Stmt)\s*:', content)

    with open(rust_output_file, 'w') as f:
        for class_name in class_names:
            rust_struct = f"""    generate!("taichi::lang::{class_name}")
"""
            f.write(rust_struct)

if __name__ == "__main__":
    generate_stmts('./stmts_include.rs')