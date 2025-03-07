use std::path::PathBuf;
use walkdir::WalkDir;

pub fn main() {
let dir = env!("CARGO_MANIFEST_DIR");

    let mut inputs = vec![];
    WalkDir::new(PathBuf::from_iter([dir, "src"]))
        .into_iter()
        .filter_map(|v| v.ok())
        .for_each(|entry|
            match entry.file_type().is_file() {
                true => inputs.append(&mut vec![entry.into_path()]),
                false => (),
            } 
        );

    let output = PathBuf::from_iter([dir, "frontend/src/types/models.d.ts"]);

    tsync::generate_typescript_defs(inputs, output, false, true);
}
