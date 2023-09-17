use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
    process::Command,
};

// use status::cc
// use crate::status::{StatusOfAppending::{Mdtext, Uml}};
use crate::{
    message_alert,
    status::md_status_mod::StatusOfAppending::{Mdtext, Uml},
};
use rand::Rng;
use regex::Regex;

fn md_file_name(name: &str) -> String {
    let mut name = name.replace('#', "");
    name = name.replace(' ', "");
    name = name.replace(':', "_");
    name = name.replace('[', "");

    let re = Regex::new(r"[^0-9.]").unwrap();
    let name = re.replace_all(&name, "");
    // name.replace(']', "").replace('/', "_")
    name.to_string()
}

fn summary_name(name: &str) -> String {
    let _ = fs::create_dir_all("./mdBook_html_files/src");
    let file_exists = Path::new("./mdBook_html_files/src/SUMMARY.md").exists();
    // let md_file_name = md_file_name(&name);
    let mut link: String = "".to_string();
    let mut tab: String = String::new();
    let mut list_name: String = String::new();
    if name.contains('#') {
        let cnt = get_line_marker_cnt(&name);
        let prepare_name = name.replace('#', "");
        list_name = prepare_name.clone().splitn(3, ' ').collect::<Vec<&str>>()[2].to_string();
        link = prepare_name.splitn(2, ' ').collect::<Vec<&str>>()[1].to_string();
        link = link.replace(" ", "-");
        link = link.replace(".", "");
        // link = link.trim().split_once(' ').unwrap().1.to_string();
        for _ in 1..cnt {
            tab.push('\t');
        }
        tab.push('-');
    }
    if !file_exists {
        format!("{} [{}](chapter_01.md)", tab, list_name)
    } else {
        format!(
            "{} [{}](#{})",
            tab,
            list_name,
            link.replace('/', "")
                .to_lowercase()
                .replace('+', "")
                .replace(':', "")
                .replace('[', "")
                .replace(']', "")
        )
    }
}

fn summary_write_append(contents: &str) {
    let summary = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./mdBook_html_files/src/SUMMARY.md");

    match summary {
        Ok(mut summary_file) => {
            writeln!(summary_file, "{}", contents).expect("Failed to write to the file");
            summary_file.flush().expect("Failed to flush the file");
        }
        Err(err) => {
            println!("summary file does not exist");
            message_alert(&err.to_string());
        }
    }
}

fn file_write(content: &String) {
    let subject = content.lines().next().to_owned().unwrap();
    summary_write_append(&summary_name(subject));

    // let subject = md_file_name(subject);
    //later
    // let index_chapter = format!("./mdBook_html_files/src/chapter_01.md{}", "");

    let index_chapter = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./mdBook_html_files/src/chapter_01.md");

    match index_chapter {
        Ok(mut index) => {
            writeln!(index, "{}", content).expect("Failed to write to the file");
            index.flush().expect("Failed to flush the file");
        }
        Err(err) => {
            message_alert(&err.to_string());
        }
    }
}

fn get_line_marker_cnt(line: &str) -> u8 {
    let mut cnt = 0;
    for n in line.chars() {
        if n == '#' {
            cnt += 1;
        }
    }
    cnt
}

fn append_content(line: String, path: String) {
    let last_index = get_the_last_line_of_index(path.clone());
    // message_alert("append content start");
    let mut img_files: Vec<String> = vec![];
    match File::open(path) {
        Ok(file) => {
            let buf = BufReader::new(file);

            let mut can_append: bool = false;
            let mut contents = "".to_string();

            let first_marker_cnt = get_line_marker_cnt(&line);
            let mut the_point_of_add_line = 0;
            let mut buf_line_index = 0;
            let mut uml_hash_name = String::new();
            let mut status_of_md = Mdtext;
            for (i, n) in buf.lines().enumerate() {
                // print_message(&line);
                let buf_line = n.unwrap();

                buf_line_index = i;
                if buf_line.contains(&line) {
                    can_append = true;
                    the_point_of_add_line = i;
                    contents.push_str(&buf_line);
                    contents.push('\n');
                }

                if can_append && i > the_point_of_add_line {
                    if buf_line.contains('#') && buf_line.starts_with('#')
                    // && get_line_marker_cnt(&buf_line) <= first_marker_cnt
                    {
                        file_write(&contents);
                        break;
                    }

                    let mut subject = contents.lines().next().to_owned().unwrap().to_string();
                    subject = subject.replace('#', "");
                    subject = subject.replace('.', "");
                    subject = subject.replace(' ', "");
                    if buf_line.contains("```plantuml") {
                        uml_hash_name = generate_random_hash();
                        contents.push_str(&format!("<img src=./{}.svg>", uml_hash_name.clone()));
                        contents.push('\n');
                        contents.push('\n');
                        status_of_md = Uml;
                    }

                    match status_of_md {
                        Uml => {
                            if buf_line == "```" {
                                status_of_md = Mdtext;
                            }
                            // message_alert("plantuml message occur");
                            append_uml(buf_line.to_string(), uml_hash_name);
                        }
                        Mdtext => {
                            contents.push_str(&buf_line);
                            contents.push('\n');
                        }
                    }
                }
            }

            if buf_line_index == last_index {
                file_write(&contents);
            }
        }
        Err(err) => {
            message_alert(&err.to_string());
        }
    };
}

pub fn file_read(path: String) {
    message_alert("It takes some time depends on the file size....wait for 2 min!!!!!!");
    let mut does_first_line_marker_not_contain = false;
    let mut contents = "".to_string();

    if let Ok(file) = File::open(path.clone()) {
        let reader = BufReader::new(file);
        for (i, n) in reader.lines().enumerate() {
            let line = n.unwrap();

            if line.contains('#') && line.starts_with('#') {
                if does_first_line_marker_not_contain {
                    does_first_line_marker_not_contain = false;
                }

                append_content(line.clone(), path.clone());
            } else if i == 0 {
                does_first_line_marker_not_contain = true;
            }
            if does_first_line_marker_not_contain {
                contents.push_str(&line);
                contents.push('\n');
            }
        }
    } else {
        message_alert("file is not exist.");
    }
}

fn get_the_last_line_of_index(path: String) -> usize {
    let file = File::open(path).unwrap();

    BufReader::new(file).lines().count() - 1
}

fn generate_random_hash() -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..8).map(|_| rng.gen()).collect();
    let hash = md5::compute(random_bytes);

    format!("{:x}", hash)
}

fn create_uml_image(filename: String) {
    let mut command = Command::new("java");

    command
        .arg("-jar")
        .arg("./plantuml.jar")
        .arg("-tsvg")
        .arg("-nometadata")
        .arg("-charset")
        .arg("UTF-8");

    command.arg(filename);

    // message_alert(&format!("command : {:?}", command));

    match command.status() {
        Ok(done) => {
            let mut msg = done.to_string();
            msg.push_str(" : img created from jar file");
            // message_alert(&msg);
        }
        Err(e) => {
            let msg = e.to_string();
            message_alert(&msg);
        }
    }
}

fn append_uml(mut content: String, mut file_name: String) {
    file_name = format!("./mdBook_html_files/src/{}.txt", file_name);

    if content.starts_with("@startuml") {
        content = "@startuml".to_string();
    }
    let uml_append_tmp_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name.clone());

    match uml_append_tmp_file {
        Ok(mut tmp_file) => {
            if let Err(err) = writeln!(tmp_file, "{}", content.clone()) {
                message_alert("file append fail");
                message_alert(&err.to_string());
            } else {
                // message_alert(&content);
            }
        }
        Err(e) => {
            message_alert(&e.to_string());
        }
    };

    if content.contains("@enduml") {
        // message_alert(&file_name);
        create_uml_image(file_name);
    }
}
