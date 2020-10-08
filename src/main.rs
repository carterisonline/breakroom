extern crate fs_extra;
extern crate markdown;

use std::env;

fn main() {
    let mut temp_c: usize = 0;

    let args: Vec<String> = env::args().collect();

    let options = fs_extra::dir::DirOptions { depth: 3 };
    let dir_content = fs_extra::dir::get_dir_content2(&args[1], &options)
        .ok()
        .unwrap();
    for filename in dir_content.files {
        if filename.ends_with(".md") {
            fs_extra::dir::create(&args[2], false);
            fs_extra::file::write_all(
                &filename,
                fs_extra::file::read_to_string(&filename)
                    .ok()
                    .unwrap()
                    .as_str()
                    .split(' ')
                    .enumerate()
                    .map(|(c, s)| {
                        temp_c = c;
                        if s.starts_with("[[") {
                            return format!("**PAGE {}**", &temp_c);
                        }
                        s.to_string()
                    })
                    .collect::<Vec<String>>()
                    .into_iter()
                    .collect::<String>()
                    .as_str(),
            );
        }
    }
}
