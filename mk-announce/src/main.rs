use std::{
    fs::{self, File},
    path::Path,
};

use argh::FromArgs;
use eyre::{Context, Result};
use handlebars::Handlebars;
use opord_parse::{
    activity::{ActivityType, LabAudience, PTDay},
    opord_parser::OpordParser,
};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::week_msg::{Activity, WeekMsg};

mod week_msg;

const UNKNOWN: &str = "[UNKNOWN]";

#[derive(FromArgs)]
#[argh(description = "AFROTC Announcement Generator")]
struct Options {
    #[argh(
        positional,
        description = "path to the OPORD text file to parse. Generates a JSON file of the same name, containing the parsed result."
    )]
    opord: Option<String>,

    #[argh(
        option,
        short = 'm',
        description = "generate the announcement html with the specified JSON file."
    )]
    json_file: Option<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let opts: Options = argh::from_env();

    if let Some(opord_path) = opts.opord {
        // read the opord
        let opord = fs::read_to_string(&opord_path).context("Failed to open the supplied OPORD")?;
        let msg = parse_and_export_opord(&opord)?;
        export_msg(&opord_path, &msg)?;
        generate_announcements(&msg)?;
    }

    if let Some(json_path) = opts.json_file {
        let msg = serde_json::from_str(&fs::read_to_string(json_path)?)?;
        generate_announcements(&msg)?;
    }

    Ok(())
}

fn export_msg(opord_path: &str, msg: &WeekMsg) -> Result<()> {
    let json_path = Path::new(opord_path).with_extension("json");

    info!(
        "Exporting data to {}",
        json_path.file_name().unwrap().to_str().unwrap()
    );

    fs::write(json_path, serde_json::to_string_pretty(&msg)?)?;
    Ok(())
}

fn no_escape(x: &str) -> String {
    x.to_string()
}

fn generate_announcements(msg: &WeekMsg) -> Result<()> {
    info!("Rendering announcements");

    let mut reg = Handlebars::new();
    reg.register_escape_fn(no_escape);
    reg.register_template_file("template", "mk-announce/www/index.hbs")
        .context("could not open the handlebars file")?;
    reg.set_strict_mode(true);

    let output_path = "mk-announce/www/render.html";

    let mut output_file =
        File::create(output_path).context(format!("Could not create file {}", output_path))?;
    reg.render_to_write("template", &msg, &mut &mut output_file)?;

    info!("Created announcements at {}", output_path);

    Ok(())
}

fn convert_activity(a: &ActivityType) -> Activity {
    let name;
    let event = UNKNOWN;
    let deets;

    match a {
        ActivityType::Unknown(x) => {
            name = UNKNOWN;
            deets = x;
        }
        ActivityType::PT(day, details) => {
            name = match day {
                PTDay::MT => "M/T PT",
                PTDay::WTH => "W/TH PT",
            };

            deets = details;
        }
        ActivityType::LLAB(audience, details) => {
            name = match audience {
                LabAudience::GMC => "GMC LLAB",
                LabAudience::POC => "POC LLAB",
                LabAudience::Joint => "LLAB",
            };

            deets = details;
        }
        ActivityType::MULLAB(details) => {
            name = "MULLAB";
            deets = details;
        }
    }

    Activity::new(
        name.to_string(),
        event.to_string(),
        deets.location().to_string(),
        deets.uniform().to_string(),
    )
}

fn parse_and_export_opord(opord: &str) -> Result<WeekMsg> {
    let parser = OpordParser::new(opord);
    let res = parser.parse()?;

    let activities = res.activities();

    info!("Parsed {} activities", activities.len());

    let converted: Vec<Activity> = activities
        .iter()
        .map(|x| {
            let conv_activ = convert_activity(x);
            info!("{}", &conv_activ);
            conv_activ
        })
        .collect();

    Ok(WeekMsg::new(
        res.week_num(),
        UNKNOWN.to_string(),
        converted,
        vec![],
    ))
}
