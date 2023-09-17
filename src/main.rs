mod create_mdbook;

mod create_mdfiles;
mod status;
use std::env::SplitPaths;
use std::fs;
use std::os::unix::process;
use std::process::Command;

//function that write_mdFile
use create_mdfiles::file_read;

//function that related with mdBook
use create_mdbook::*;

//dialog
use native_dialog::FileDialog;
use native_dialog::MessageDialog;
use native_dialog::MessageType;

fn message_alert(str: &str) {
    // MessageDialog::new()
    //     .set_type(MessageType::Info)
    //     .set_title(&str)
    //     .set_text(&str)
    //     .show_alert()
    //     .unwrap();
    println!("{}", str);
}

fn print_message(str: &str) {
    println!(" ------------{}", str);
}

fn custom_print(f_path: &str) {
    let content = fs::read_to_string(f_path);

    let modified_content: Vec<String> = content
        .unwrap()
        .lines()
        .enumerate()
        .map(|(index, line)| {
            if !line.contains("break-before") {
                line.to_string()
            } else {
                "".to_string()
            }
        })
        .collect();

    let output = modified_content.join("\n");
    println!("### {:?}", output.lines().count());
    fs::write(f_path, output.clone()).unwrap();
}
fn main() {
    message_alert("book start create");

    create_mdbook();

    //window
    // let file_name = FileDialog::new()
    //     .set_location("~/")
    //     // .add_filter("md", &["md"])
    //     .show_open_single_file();
    // let path = file_name.expect("can not fine").expect("error");

    // message_alert(path.to_str().unwrap());

    // file_read(path.to_str().unwrap().to_string());

    // message_alert();
    //linux
    file_read("./message_specification/message_specification.md".to_string());

    let chapter_path = "./mdBook_html_files/book/chapter_01.html";
    let mut output = "".to_string();
    match build_mdbook() {
        Ok(_) => {
            let f_path = "./mdBook_html_files/book/index.html";

            let content = fs::read_to_string(f_path);

            let modified_content: Vec<String> = content
                .unwrap()
                .lines()
                .enumerate()
                .map(|(index, line)| {
                    if 76 < index && index < 100 {
                        // 0-based index, so 71 corresponds to line 72
                        // Using a regex to find patterns like "#abcd.html" and replace with "#abcd"
                        println!("### line contains :{}", line);

                        let result = line.replace(".html", "");
                        let result = result.replace("chapter_01", "chapter_01.html");
                        return result;
                    } else {
                        line.to_string()
                    }
                })
                .collect();

            output = modified_content.join("\n");
            println!("### {:?}", output.lines().count());
            fs::write(f_path, output.clone()).unwrap();
            // fs::write(chapter_path, output).unwrap();
        }
        Err(e) => {
            message_alert(&e.to_string());
        }
    }

    println!("### {:?}", output.lines().count());
    fs::write(chapter_path, output).unwrap();

    custom_print("./mdBook_html_files/book/print.html");
    message_alert("everything is done");
}
