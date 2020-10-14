extern crate chrono;
extern crate fern;

use fern::colors::ColoredLevelConfig;

macro_rules! color {
    ($color:ident) => {
        format_args!("\x1B[{}m", fern::colors::Color::$color.to_fg_str())
    }
}

pub fn setup_logs(colors: ColoredLevelConfig) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "\n{color_gray}[{color_white}{crate_name}{color_gray}] {color_red}@ {color_gray}[{color_red}{date}{color_gray}][{color_red}{time}{color_gray}]{color_white}: {color_gray}[{loglevel}{color_gray}] {color_white}=> {color_yellow}{message}{color_white}",
                color_gray = color!(BrightBlack),
                color_red = color!(BrightRed),
                color_white = color!(White),
                color_yellow = color!(BrightYellow),
                crate_name = record.target(),
                date = chrono::Local::now().format("%Y-%m-%d"),
                loglevel = colors.color(record.level()),
                message = message,
                time = chrono::Local::now().format("%H:%M.%S")
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}