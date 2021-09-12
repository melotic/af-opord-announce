use argh::FromArgs;
use eyre::{Context, Result};
use opord_parse::week_msg::WeekMsg;
use std::fs;

#[derive(FromArgs)]
#[argh(description = "copies announcements from one JSON file to another")]
struct Options {
    #[argh(
        positional,
        description = "the JSON file to copy the announcements from"
    )]
    src: String,

    #[argh(positional, description = "the JSON file to copy the announcements to")]
    dst: String,
}

fn open_msg(path: &str) -> Result<WeekMsg> {
    let msg: WeekMsg = serde_json::from_str(
        &fs::read_to_string(path).wrap_err_with(|| format!("couldn't open json file {}", path))?,
    )
    .wrap_err_with(|| format!("Couldnt deserialize {}", path))?;

    Ok(msg)
}
fn main() -> Result<()> {
    let opts: Options = argh::from_env();

    let src = open_msg(&opts.src)?;
    let mut dst = open_msg(&opts.dst)?;

    *dst.announcements_mut() = src.announcements().to_vec();

    fs::write(
        opts.dst,
        serde_json::to_string_pretty(&dst).context("failed to serialize new JSON")?,
    )
    .context("Failed to overwrite dest JSON")?;

    Ok(())
}
