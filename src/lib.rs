use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};

struct MVOBJ {
    old: String,
    new: String,
}

pub fn run(files: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    files.remove(0);
    files.retain(|p| *p != "." && *p != "..");
    let input = prepare_input(files);

    let child = Command::new("vipe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child.stdin.as_ref().unwrap().write_all(input.as_bytes())?;
    let output = child.wait_with_output()?;

    if output.status.code().unwrap_or(1) != 0 {
        panic!("unexpected error quitting..")
    }

    let mvobjs = prepare_objs_list(input, String::from_utf8(output.stdout)?);

    for o in mvobjs {
        // if &o.old != &o.new {
        //     // Application error: Invalid cross-device link (os error 18)
        //     std::fs::rename(&o.old, if &o.new == "" { "/tmp/" } else { &o.new })?;
        // }

        if &o.old != &o.new {
            let res = Command::new("mv")
                .args(["-n", &o.old, if &o.new == "" { "/tmp/" } else { &o.new }])
                .output()
                .expect("");
            if !res.status.success() {
                panic!(
                    "error moving: {} {:?}",
                    o.old,
                    String::from_utf8(res.stderr)?
                )
            }
        }
    }

    Ok(())
}

fn prepare_input(files: &mut Vec<String>) -> String {
    let mut input = String::new();

    for (i, line) in files.iter().enumerate() {
        input.push_str(&(i + 1).to_string()[..]);
        input.push_str("|");
        input.push_str(&line);
        input.push_str("\n");
    }

    input
}

fn prepare_objs_list(input: String, output: String) -> Vec<MVOBJ> {
    let mut mvobjs = Vec::new();

    for i in 0..input.lines().count() {
        let p = format!("{}|", &i + 1);
        let mut input_str = input
            .lines()
            .find(|x| x.contains(&p))
            .unwrap_or("")
            .to_string();
        let mut output_str = output
            .lines()
            .find(|x| x.contains(&p))
            .unwrap_or("")
            .to_string();

        mvobjs.push(MVOBJ {
            old: {
                let pipe_offset = input_str.find("|").unwrap_or(0);
                input_str.replace_range(..pipe_offset + 1, "");
                input_str
            },
            new: if output_str == "" {
                output_str
            } else {
                let pipe_offset = output_str.find("|").unwrap_or(0);
                output_str.replace_range(..pipe_offset + 1, "");
                output_str
            },
        });
    }

    mvobjs
}
