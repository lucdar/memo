use chrono::Local;
use edit::edit_file;
use std::fs;
use std::{io::Write, path::PathBuf};

pub fn compose(memos_dir: PathBuf, title: Option<String>) -> Result<(), std::io::Error> {
    // dbg!("in compose", memos_dir, title);
    let (mut file, filepath) = edit::Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile_in(&memos_dir)?
        .keep()?;
    // dbg!(&filepath);

    let template = match title.as_deref() {
        Some(k) => format!("# {k}\n"),
        None => String::new(),
    };

    file.write_all(template.as_bytes())?;
    edit_file(&filepath)?;

    // Use current time as title if no title specified
    let mut filename = title.unwrap_or(
        Local::now()
            .format(
                //TODO: add date format as a parameter
                "%b %d, %Y - %I:%M%P",
            )
            .to_string(),
    );
    // dbg!(&filename);

    for attempt in 0.. {
        let number_suffix = format!(" ({})", attempt);
        let new_filename = if attempt != 0 {
            filename.clone() + &number_suffix
        } else {
            filename.clone()
        };
        let mut dest = memos_dir.join(&new_filename);
        dest.set_extension("md");
        if dest.exists() {
            continue;
        }
        fs::rename(&filepath, &dest)?;
        break;
        // dbg!(&dest);
    }

    Ok(())
}
