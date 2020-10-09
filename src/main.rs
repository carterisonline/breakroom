extern crate fs_extra;
extern crate markdown;

use std::env;

const ERROR_BACKLINK_MISMATCH: &str = "If you're seeing this, congratulations! Your file contained malformed backlinks.\n\nYour content that was successfully converted is still saved in this folder. Here's where the issue was:\n\n";

fn main() {
    let mut temp_c: usize = 0;
    let mut temp_content_word: String = String::new();
    let mut temp_content_line: String = String::new();

    let mut entry: usize = 0;

    let args: Vec<String> = env::args().collect();

    let options = fs_extra::dir::DirOptions { depth: 3 };
    let all_files: Vec<(usize, String)> = fs_extra::dir::get_dir_content2(&args[1], &options)
        .ok()
        .unwrap()
        .files
        .into_iter()
        .enumerate()
        .collect();

    fs_extra::dir::create(&args[2], false).unwrap();

    println!("{:?}", all_files);

    for (fileid, filename) in &all_files {
        //println!("{}", &filename);
        if filename.ends_with(".md") {
            let mut words: Vec<String> = vec![];
            fs_extra::file::write_all(
                format!("{}/{}", &args[2], filename),
                fs_extra::file::read_to_string(&filename)
                    .ok()
                    .unwrap()
                    .split('\n')
                    .enumerate()
                    .map(|(linenum, s)| {
                        let mut current_index: usize = 0;
                        if !&s.contains("[[") || !&s.contains("]]") {
                            return String::from(format!("{}\n", s));
                        }
                        temp_content_line = String::new();
                        let start: Vec<usize> = s.match_indices("[[").map(|(i, _s)| i).collect();
                        let end: Vec<usize> = s.match_indices("]]").map(|(i, _s)| i).collect();

                        if &start.len() != &end.len() {
                            let options_2 = fs_extra::dir::CopyOptions {
                                copy_inside: true,
                                ..Default::default()
                            };
                            fs_extra::dir::move_dir(
                                &args[2],
                                format!("{}.failed", &args[2]),
                                &options_2,
                            )
                            .unwrap();
                            fs_extra::file::write_all(
                                format!("{}.failed/.igneous_error", &args[2]),
                                format!(
                                    "{}{}: Line {}",
                                    ERROR_BACKLINK_MISMATCH,
                                    &filename,
                                    &linenum + 1
                                )
                                .as_str(),
                            )
                            .unwrap();
                            panic!(
                                "Error parsing `{}: Line {}` => Unusual backlink patterns",
                                &filename,
                                &linenum + 1
                            );
                        }

                        start.iter().enumerate().for_each(|(j, l)| {
                            let filenamesingle = s.get((l + 2)..end[j]).unwrap();
                            all_files.iter().for_each(|(v, t)| {
                                if t.ends_with(format!("{}.md", filenamesingle.trim()).as_str()) {
                                    entry = *v;
                                    temp_content_line.push_str(s.get(current_index..*l).unwrap_or(""));
                                    temp_content_line
                                        .push_str(format!("__[Page {}]__", entry).as_str());
                                    if j == start.len() - 1 {
                                        temp_content_line
                                            .push_str(s.get((end[j] + 2)..(s.len() - 1)).unwrap_or(""));
                                    }
                                    current_index = end[j] + 2;
                                }
                            });
                        });

                        &temp_content_line.push('\n');
                        temp_content_line.clone()
                    })
                    .collect::<String>()
                    .as_str(),
            );
            //.unwrap();
            //println!("DONE!");
        }
    }
}
