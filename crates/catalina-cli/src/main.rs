//! The Catalina command line provides tools for working with
//! the Catalina hardware devices, such as modules, module
//! carrier boards, and kits.

use clap::builder::styling::Styles;
use clap::{Parser, Subcommand, crate_description, crate_version};

use color_eyre::config::HookBuilder;
use color_eyre::eyre::{EyreHandler, InstallError, Result};

use owo_colors::OwoColorize;

// Parts of this error handling approach are inspired by Rachel Mant (dragonmux)'s work on bmputil-cli:
//  see: https://github.com/blackmagic-debug/bmputil

type EyreHookFunc =
    Box<dyn Fn(&(dyn std::error::Error + 'static)) -> Box<dyn EyreHandler> + Send + Sync + 'static>;
type PanicHookFunc = Box<dyn Fn(&std::panic::PanicHookInfo<'_>) + Send + Sync + 'static>;

struct CatalinaHook {
    inner_hook: EyreHookFunc,
}

struct CatalinaPanic {
    inner_hook: PanicHookFunc,
}

struct CatalinaHandler {
    inner_handler: Box<dyn EyreHandler>,
}

impl CatalinaHook {
    fn build_handler(&self, error: &(dyn std::error::Error + 'static)) -> CatalinaHandler {
        CatalinaHandler {
            inner_handler: (*self.inner_hook)(error),
        }
    }

    pub fn install(self) -> Result<(), InstallError> {
        color_eyre::eyre::set_hook(self.into_eyre_hook())
    }

    pub fn into_eyre_hook(self) -> EyreHookFunc {
        Box::new(move |err| Box::new(self.build_handler(err)))
    }
}

impl CatalinaPanic {
    pub fn install(self) {
        std::panic::set_hook(self.into_panic_hook());
    }

    pub fn into_panic_hook(self) -> PanicHookFunc {
        Box::new(move |panic_info| {
            self.print_header();
            (*self.inner_hook)(panic_info);
            self.print_footer();
        })
    }

    fn print_header(&self) {
        eprintln!("------------[ ✂ cut here ✂ ]------------");
        eprintln!(
            "Unhandled crash in catalina-cli v{} ({})",
            crate_version!(),
            std::env::consts::OS
        );
        eprintln!();
    }

    fn print_footer(&self) {
        eprintln!();
        eprintln!(
            "{}",
            "Please include all lines down to this one from the cut here".yellow()
        );
        eprintln!(
            "{}",
            "marker, and report this issue to our issue tracker at".yellow()
        );
        eprintln!("https://github.com/northernpaws/catalina/issues");
    }
}

impl EyreHandler for CatalinaHandler {
    fn debug(
        &self,
        error: &(dyn std::error::Error + 'static),
        fmt: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        writeln!(fmt, "------------[ ✂ cut here ✂ ]------------")?;
        write!(fmt, "Unhandled crash in Catalina-cli v{}", crate_version!())?;
        self.inner_handler.debug(error, fmt)?;
        writeln!(fmt)?;
        writeln!(fmt)?;
        writeln!(
            fmt,
            "{}",
            "Please include all lines down to this one from the cut here".yellow()
        )?;
        writeln!(
            fmt,
            "{}",
            " marker, and report this issue to our issue tracker at".yellow()
        )?;
        write!(fmt, "https://github.com/northernpaws/catalina/issues")
    }

    fn track_caller(&mut self, location: &'static std::panic::Location<'static>) {
        self.inner_handler.track_caller(location);
    }
}

fn install_error_handler() -> Result<()> {
    // Grab us a new default handler
    let default_handler = HookBuilder::default();
    // Turn that into a pair of hooks - one for panic, and the other for errors
    let (panic_hook, eyre_hook) = default_handler.try_into_hooks()?;

    // Make an instance of our custom handler, paassing it the panic one to do normal panic
    // handling with, so we only have to deal with our additions, and install it
    CatalinaPanic {
        inner_hook: panic_hook.into_panic_hook(),
    }
    .install();

    // Make an instance of our custom handler, passing it the default one to do the main
    // error handling with, so we only have to deal with our additions, and install it
    CatalinaHook {
        inner_hook: eyre_hook.into_eyre_hook(),
    }
    .install()?;
    Ok(())
}

/// Clap v3 style (approximate)
/// See https://stackoverflow.com/a/75343828
fn style() -> clap::builder::Styles {
    Styles::styled()
        .usage(
            anstyle::Style::new()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow)))
                .bold(),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
}

#[derive(Parser)]
#[command(
	version,
	about = format!("{} v{}", crate_description!(), crate_version!()),
	styles(style()),
	disable_colored_help(false),
	arg_required_else_help(true)
)]
struct CliArguments {
    #[command(subcommand)]
    pub subcommand: ToplevelCommmands,
}

#[derive(Subcommand)]
enum ToplevelCommmands {}

fn main() -> Result<()> {
    install_error_handler()?;
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let cli_args = CliArguments::parse();

    Ok(())
}
