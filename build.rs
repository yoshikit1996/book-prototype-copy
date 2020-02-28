use std::path::Path;

// protoファイルのファイルパスの一覧を返す関数
fn paths(root_path: &str) -> Vec<String> {
    Path::new(root_path).read_dir().expect("read_dir call failed")
        .into_iter()
        .flat_map(|dir_entry_or_error| {
            match dir_entry_or_error {
                Ok(_dir_entity) if _dir_entity.path().to_str().unwrap().ends_with(".proto") =>
                    vec![_dir_entity.path().to_str().unwrap().to_string()],
                Ok(_dir_entity) if _dir_entity.path().is_dir() =>
                    paths(_dir_entity.path().to_str().unwrap()),
                Ok(_) =>
                    vec![],
                Err(e) => panic!(e)
            }
        }).collect()
}

fn main() {
    for _path in paths("./src/") {
        tonic_build::compile_protos(_path).unwrap();
    }
}