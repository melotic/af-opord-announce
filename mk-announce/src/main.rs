use std::{
    fs::{self, File},
    path::Path,
};

use argh::FromArgs;
use chrono::{Datelike, Local, NaiveDate};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use eyre::{Context, Result};
use handlebars::{Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};
use opord_parse::{
    activity::{ActivityType, LabAudience, PTDay},
    opord_parser::OpordParser,
    week_msg::{Activity, WeekMsg},
};
use tracing::info;
use tracing_subscriber::EnvFilter;

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

fn format_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let date_str = h
        .param(0)
        .ok_or_else(|| RenderError::new("param 0 is req'd"))?
        .value()
        .render();

    info!("{}", date_str);

    let date = NaiveDate::parse_from_str(&date_str, "%m-%d-%Y")
        .map_err(|_| RenderError::new("invalid date"))?;

    let today = Local::today();
    let today = NaiveDate::from_ymd(today.year(), today.month(), today.day());

    let diff = date - today;
    let ht = HumanTime::from(diff);

    out.write(&ht.to_text_en(Accuracy::Precise, Tense::Present))?;

    Ok(())
}

fn generate_announcements(msg: &WeekMsg) -> Result<()> {
    info!("Rendering announcements");

    let mut reg = Handlebars::new();
    reg.register_escape_fn(no_escape);
    reg.register_helper("format_date", Box::new(format_helper));
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
                PTDay::Remedial => "Hero Workout",
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
        vec![],
        converted,
        vec![],
    ))
}
