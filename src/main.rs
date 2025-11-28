//! Genkan - A fast static site generator for creating Linktree-like pages
//!
//! Genkan (玄関, meaning "entrance") generates beautiful, responsive link pages
//! from simple TOML configuration files. The output is a single self-contained
//! HTML file with embedded CSS and JavaScript, perfect for self-hosting.
//!
//! # Quick Start
//!
//! ```bash
//! # Initialize a new project
//! genkan init my-links
//!
//! # Edit config.toml with your links
//! cd my-links
//!
//! # Generate the page
//! genkan build
//!
//! # Open output/index.html in your browser
//! ```

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use genkan::{config, generator};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "genkan")]
#[command(about = "A static site generator for creating Linktree-like pages", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate the static page from config
    Build {
        /// Path to config file
        #[arg(short, long, default_value = "config.toml")]
        config: PathBuf,

        /// Output directory
        #[arg(short, long, default_value = "output")]
        output: PathBuf,
    },
    /// Initialize a new Genkan project
    Init {
        /// Project directory (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Validate the config file
    Validate {
        /// Path to config file
        #[arg(short, long, default_value = "config.toml")]
        config: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Build { config, output }) => {
            build_site(config, output)?;
        }
        Some(Commands::Init { path }) => {
            init_project(path)?;
        }
        Some(Commands::Validate { config }) => {
            validate_config(config)?;
        }
        None => {
            // Default behavior: build with default settings
            build_site(PathBuf::from("config.toml"), PathBuf::from("output"))?;
        }
    }

    Ok(())
}

/// Builds a static site from the configuration file
///
/// # Arguments
///
/// * `config_path` - Path to the TOML configuration file
/// * `output_dir` - Directory where the generated HTML will be saved
///
/// # Returns
///
/// * `Ok(())` if the site was generated successfully
/// * `Err(anyhow::Error)` if configuration loading, validation, or generation failed
fn build_site(config_path: PathBuf, output_dir: PathBuf) -> Result<()> {
    println!("Building site...\n");

    // Load configuration
    println!("Loading config from: {}", config_path.display());
    let config = config::Config::from_file(&config_path).context("Failed to load configuration")?;

    // Validate configuration
    config
        .validate()
        .context("Configuration validation failed")?;
    println!("Config validated\n");

    // Find theme path
    let theme_path =
        generator::find_theme_path(&config.theme.name).context("Failed to find theme")?;
    println!(
        "Using theme: {} ({})",
        config.theme.name,
        theme_path.display()
    );

    // Create output path
    let output_path = output_dir.join("index.html");

    // Generate site
    let generator = generator::Generator::new(config, theme_path, output_path.clone());
    generator.generate().context("Failed to generate site")?;

    println!(
        "\nSuccess! Your link page is ready at: {}",
        output_path.display()
    );
    println!("\nTip: Open the file in your browser to see your page!");

    Ok(())
}

/// Initializes a new Genkan project with default configuration
///
/// Creates a new project directory structure with:
/// - config.toml (default configuration file)
/// - themes/ (directory for custom themes)
/// - output/ (directory for generated HTML)
///
/// # Arguments
///
/// * `path` - Directory path where the project will be initialized
///
/// # Returns
///
/// * `Ok(())` if initialization was successful
/// * `Err(anyhow::Error)` if the directory creation failed or config.toml already exists
fn init_project(path: PathBuf) -> Result<()> {
    println!("Initializing new Genkan project...\n");

    // Create project directory if it doesn't exist
    if !path.exists() {
        std::fs::create_dir_all(&path).context("Failed to create project directory")?;
    }

    // Create config file
    let config_path = path.join("config.toml");
    if config_path.exists() {
        anyhow::bail!("config.toml already exists! Remove it first if you want to reinitialize.");
    }

    let default_config = concat!(
        "# Genkan Configuration File\n",
        "# This file controls your link page content and appearance\n",
        "\n",
        "[profile]\n",
        "name = \"Your Name\"\n",
        "bio = \"Welcome to my link page!\"\n",
        "# Avatar can be a URL or local path (relative to config.toml)\n",
        "avatar = \"https://via.placeholder.com/150\"\n",
        "# Optional: background image or gradient\n",
        "# background = \"linear-gradient(135deg, #667eea 0%, #764ba2 100%)\"\n",
        "\n",
        "[theme]\n",
        "# Theme name (currently supports: simple)\n",
        "name = \"simple\"\n",
        "# Primary color for buttons and accents\n",
        "primary_color = \"#000000\"\n",
        "# Secondary color for accents\n",
        "secondary_color = \"#000000\"\n",
        "# Background color (can be overridden by profile.background)\n",
        "background_color = \"#ffffff\"\n",
        "# Button style: rounded, pill, square\n",
        "button_style = \"rounded\"\n",
        "# Font family\n",
        "font_family = \"system-ui, -apple-system, sans-serif\"\n",
        "# Spacing between link buttons\n",
        "link_spacing = \"24px\"\n",
        "# Color domains - granular control over text colors\n",
        "header_color = \"#000000\"\n",
        "bio_color = \"rgba(0, 0, 0, 0.7)\"\n",
        "link_title_color = \"#000000\"\n",
        "link_description_color = \"rgba(0, 0, 0, 0.6)\"\n",
        "\n",
        "[meta]\n",
        "# Page metadata\n",
        "title = \"My Links\"\n",
        "description = \"All my important links in one place\"\n",
        "# Optional: favicon (URL or local path like \"./favicon.ico\")\n",
        "favicon = \"\"\n",
        "# Optional: Add custom CSS\n",
        "custom_css = \"\"\n",
        "# Optional: Add analytics (Google Analytics, Plausible, etc.)\n",
        "analytics = \"\"\n",
        "\n",
        "# Define your links here\n",
        "# Each link can have: title, url (optional), icon (optional), description (optional)\n",
        "# link_type: \"block\" (default) or \"space\" (for spacing)\n",
        "# Omit url for non-clickable text blocks, omit icon for text-only\n",
        "[[links]]\n",
        "title = \"My Website\"\n",
        "url = \"https://example.com\"\n",
        "icon = \"🌐\"\n",
        "description = \"Check out my personal website\"\n",
        "link_type = \"block\"\n",
        "\n",
        "[[links]]\n",
        "title = \"GitHub\"\n",
        "url = \"https://github.com/username\"\n",
        "icon = \"https://cdn.simpleicons.org/github/000000\"\n",
        "link_type = \"block\"\n",
        "\n",
        "# Example: Spacer (creates vertical space)\n",
        "# [[links]]\n",
        "# title = \"\"\n",
        "# link_type = \"space\"\n",
        "# height = \"30px\"\n",
        "\n",
        "[[links]]\n",
        "title = \"Twitter\"\n",
        "url = \"https://twitter.com/username\"\n",
        "icon = \"🐦\"\n",
        "link_type = \"block\"\n",
    );

    std::fs::write(&config_path, default_config).context("Failed to write config file")?;
    println!("Created config.toml");

    // Create themes directory
    let themes_dir = path.join("themes");
    std::fs::create_dir_all(&themes_dir).context("Failed to create themes directory")?;
    println!("Created themes directory");

    // Create output directory
    let output_dir = path.join("output");
    std::fs::create_dir_all(&output_dir).context("Failed to create output directory")?;
    println!("Created output directory");

    println!("\nProject initialized successfully!");
    println!("\nNext steps:");
    println!("  1. Edit config.toml to add your information");
    println!("  2. Run `genkan build` to generate your page");
    println!("  3. Open output/index.html in your browser");

    Ok(())
}

/// Validates a configuration file without generating the site
///
/// Checks that:
/// - The configuration file can be parsed
/// - All required fields are present and valid
/// - The specified theme exists
///
/// # Arguments
///
/// * `config_path` - Path to the TOML configuration file to validate
///
/// # Returns
///
/// * `Ok(())` if the configuration is valid
/// * `Err(anyhow::Error)` if validation failed with details about the error
fn validate_config(config_path: PathBuf) -> Result<()> {
    println!("Validating config...\n");

    // Load configuration
    let config = config::Config::from_file(&config_path).context("Failed to load configuration")?;

    // Validate configuration
    config
        .validate()
        .context("Configuration validation failed")?;

    // Check theme exists
    let theme_path =
        generator::find_theme_path(&config.theme.name).context("Failed to find theme")?;

    println!("Configuration is valid");
    println!(
        "Theme '{}' found at: {}",
        config.theme.name,
        theme_path.display()
    );
    println!("{} link(s) configured", config.links.len());

    Ok(())
}
