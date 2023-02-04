#[macro_use]


fn main() {
 
    let index_dir = std::path::Path::new("/tmp/index/").join("tantivy-index"); 
    let index_path = index_dir.as_path();

    if !index_dir.exists() {
        _ = std::fs::create_dir_all(index_path);
    }
 
}