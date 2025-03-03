use std::path::PathBuf;

pub fn main() {
let dir = env!("CARGO_MANIFEST_DIR");

    let inputs = vec![PathBuf::from_iter([dir, "src/db/models.rs"])];
    let output = PathBuf::from_iter([dir, "frontend/src/types/models.d.ts"]);

    tsync::generate_typescript_defs(inputs, output, false, true);
}
