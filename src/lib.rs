use anyhow::{anyhow, Context, Result};
use chrono::Local;
use cli_select::{KeyCode, Select};
use edit::edit_file;
use rand::seq::SliceRandom;
use std::fs;
use std::{io::Write, path::PathBuf};


pub fn compose(memos_dir: PathBuf, title: Option<String>) -> Result<()> {
    // dbg!("in compose", memos_dir, title);
    let (mut file, filepath) = edit::Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile_in(&memos_dir).context("failed to create tempfile")?
        .keep()
        .context("failed to keep tempfile")?;
    // dbg!(&filepath);

    let template = match title.as_deref() {
        Some(k) => format!("# {k}\n"),
        None => String::new(),
    };

    file.write_all(template.as_bytes())
        .context("failed to write template to tempfile")?;
    edit_file(&filepath).context("failed to spawn editor")?;

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

    let mut dest = memos_dir.join(&filename);
    dest.set_extension("md");
    if dest.exists() {
        return Err(anyhow!("memo with title already exists"));
    }
    fs::rename(&filepath, &dest).context("failed to move file")?;
    // dbg!(&dest);

    Ok(())
}

#[derive(Debug)]
struct DisplayablePathBuf {
    pb: PathBuf,
}

impl std::fmt::Display for DisplayablePathBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pb.display().fmt(f)
    }
}

pub fn edit(memos_dir: PathBuf, random: bool) -> Result<()> {
    let mut memos: Vec<DisplayablePathBuf> = vec![];
    for memo in fs::read_dir(&memos_dir)? {
        let memo = DisplayablePathBuf { 
            pb: memo
                .context("error reading memos directory")?
                .path()
        };
        memos.push(memo);
    }

    let edit_helper = |selection| {
        edit_file(memos_dir.join(selection));
        Ok(())
    };

    if random {
        let selection = &memos
            .choose(&mut rand::thread_rng())
            .unwrap()
            .pb;
        return edit_helper(selection)
    }

    let mut select = Select::new(&memos, std::io::stdout());
    let selection = &select
        .add_up_key(KeyCode::Char('k'))
        .add_down_key(KeyCode::Char('j'))
        .start()
        .pb;

    edit_helper(selection)

    
}
