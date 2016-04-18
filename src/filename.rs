use rand::random;
use std::env;
use std::path;

pub fn rand_string() -> String {
    (0..15).map(|_| (97u8 + (random::<f32>() * 25.0) as u8) as char).collect()
}

pub fn rand_file(parentdir: &str, startwith: &str, ext: &str) -> path::PathBuf {
    let current_dir: path::PathBuf = env::current_dir().unwrap();
    let mut out_path = if parentdir != "" {
        let mut d = path::PathBuf::new();
        d.push(parentdir);
        d
    } else {
        current_dir
    };
    let filename = format!("{}{}.{}", startwith, rand_string(), ext);
    out_path.push(&filename);
    out_path
}
