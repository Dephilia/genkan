//! Configuration structures and parsing for Genkan
//!
//! This module defines all configuration structures used by Genkan to generate
//! static link pages. Configuration is loaded from TOML files and validated
//! before generation.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Root configuration structure for a Genkan site
///
/// This is the main configuration object that contains all settings
/// needed to generate a link page. It's loaded from a TOML file.
///
/// # Example
///
/// ```toml
/// [profile]
/// name = "Your Name"
/// bio = "Welcome to my link page!"
///
/// [theme]
/// name = "simple"
///
/// [meta]
/// title = "My Links"
/// description = "All my important links"
///
/// [[links]]
/// title = "My Website"
/// url = "https://example.com"
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub profile: Profile,
    pub theme: Theme,
    pub meta: Meta,
    pub links: Vec<Link>,
    #[serde(default)]
    pub dark_mode: DarkMode,
    #[serde(default)]
    pub image: ImageSettings,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub name: String,
    pub bio: String,
    #[serde(default)]
    pub social_links: Vec<SocialLink>,
    #[serde(default)]
    pub light: ProfileAssets,
    #[serde(default)]
    pub dark: ProfileAssets,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ProfileAssets {
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub background: Option<String>,
    #[serde(default)]
    pub background_image: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SocialLink {
    pub icon: String,
    pub url: String,
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Theme {
    pub name: String,
    #[serde(default = "default_button_style")]
    pub button_style: String,
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(default = "default_link_spacing")]
    pub link_spacing: String,
    #[serde(default)]
    pub typography: Typography,
    #[serde(default)]
    pub light: ThemeColors,
    #[serde(default)]
    pub dark: ThemeColors,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThemeColors {
    #[serde(default = "default_primary_color")]
    pub primary_color: String,
    #[serde(default = "default_secondary_color")]
    pub secondary_color: String,
    #[serde(default = "default_background_color")]
    pub background_color: String,
    #[serde(default = "default_header_color")]
    pub header_color: String,
    #[serde(default = "default_bio_color")]
    pub bio_color: String,
    #[serde(default = "default_link_title_color")]
    pub link_title_color: String,
    #[serde(default = "default_link_description_color")]
    pub link_description_color: String,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            primary_color: default_primary_color(),
            secondary_color: default_secondary_color(),
            background_color: default_background_color(),
            header_color: default_header_color(),
            bio_color: default_bio_color(),
            link_title_color: default_link_title_color(),
            link_description_color: default_link_description_color(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Typography {
    #[serde(default)]
    pub default: TypographyStyle,
    #[serde(default)]
    pub header: TypographyStyle,
    #[serde(default)]
    pub bio: TypographyStyle,
    #[serde(default)]
    pub link_title: TypographyStyle,
    #[serde(default)]
    pub link_description: TypographyStyle,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct TypographyStyle {
    #[serde(default)]
    pub size: Option<String>,
    #[serde(default)]
    pub font: Option<String>,
    #[serde(default)]
    pub weight: Option<String>,
    #[serde(default)]
    pub style: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub color_dark: Option<String>,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            default: TypographyStyle {
                size: Some("16px".to_string()),
                font: Some("system-ui, -apple-system, sans-serif".to_string()),
                weight: Some("normal".to_string()),
                style: Some("normal".to_string()),
                color: Some("#000000".to_string()),
                color_dark: None,
            },
            header: TypographyStyle {
                size: Some("2rem".to_string()),
                font: None,
                weight: Some("700".to_string()),
                style: Some("normal".to_string()),
                color: Some("#000000".to_string()),
                color_dark: None,
            },
            bio: TypographyStyle {
                size: Some("1.1rem".to_string()),
                font: None,
                weight: Some("normal".to_string()),
                style: Some("normal".to_string()),
                color: Some("rgba(0, 0, 0, 0.7)".to_string()),
                color_dark: None,
            },
            link_title: TypographyStyle {
                size: Some("1.1rem".to_string()),
                font: None,
                weight: Some("600".to_string()),
                style: Some("normal".to_string()),
                color: Some("#000000".to_string()),
                color_dark: None,
            },
            link_description: TypographyStyle {
                size: Some("0.9rem".to_string()),
                font: None,
                weight: Some("normal".to_string()),
                style: Some("normal".to_string()),
                color: Some("rgba(0, 0, 0, 0.6)".to_string()),
                color_dark: None,
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DarkMode {
    #[serde(default = "default_dark_mode_mode")]
    pub mode: String,
}

impl Default for DarkMode {
    fn default() -> Self {
        Self {
            mode: "disable".to_string(),
        }
    }
}

fn default_dark_mode_mode() -> String {
    "disable".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImageSettings {
    #[serde(default = "default_avatar_size")]
    pub avatar_size: u32,
    #[serde(default = "default_social_icon_size")]
    pub social_icon_size: u32,
    #[serde(default = "default_link_icon_size")]
    pub link_icon_size: u32,
    #[serde(default = "default_favicon_size")]
    pub favicon_size: u32,
}

impl Default for ImageSettings {
    fn default() -> Self {
        Self {
            avatar_size: 512,
            social_icon_size: 128,
            link_icon_size: 128,
            favicon_size: 64,
        }
    }
}

fn default_avatar_size() -> u32 {
    512
}

fn default_social_icon_size() -> u32 {
    128
}

fn default_link_icon_size() -> u32 {
    128
}

fn default_favicon_size() -> u32 {
    64
}

impl Typography {
    /// Get resolved typography values for a specific element, falling back to defaults
    pub fn resolve(
        &self,
        element: &TypographyStyle,
        legacy_color: Option<&str>,
        legacy_color_dark: Option<&str>,
    ) -> ResolvedTypography {
        ResolvedTypography {
            size: element
                .size
                .clone()
                .or_else(|| self.default.size.clone())
                .unwrap_or_else(|| "16px".to_string()),
            font: element
                .font
                .clone()
                .or_else(|| self.default.font.clone())
                .unwrap_or_else(|| "system-ui, -apple-system, sans-serif".to_string()),
            weight: element
                .weight
                .clone()
                .or_else(|| self.default.weight.clone())
                .unwrap_or_else(|| "normal".to_string()),
            style: element
                .style
                .clone()
                .or_else(|| self.default.style.clone())
                .unwrap_or_else(|| "normal".to_string()),
            color: element
                .color
                .clone()
                .or_else(|| legacy_color.map(|s| s.to_string()))
                .or_else(|| self.default.color.clone())
                .unwrap_or_else(|| "#000000".to_string()),
            color_dark: element
                .color_dark
                .clone()
                .or_else(|| legacy_color_dark.map(|s| s.to_string()))
                .or_else(|| self.default.color_dark.clone()),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ResolvedTypography {
    pub size: String,
    pub font: String,
    pub weight: String,
    pub style: String,
    pub color: String,
    pub color_dark: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Meta {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub page_url: Option<String>,
    #[serde(default)]
    pub favicon: Option<String>,
    #[serde(default)]
    pub custom_css: Option<String>,
    #[serde(default)]
    pub analytics: Option<String>,
    #[serde(default = "default_show_footer")]
    pub show_footer: bool,
    #[serde(default)]
    pub share_title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Link {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_link_type")]
    pub link_type: String,
    #[serde(default)]
    pub height: Option<String>,
}

fn default_link_type() -> String {
    "block".to_string()
}

// Default values
fn default_primary_color() -> String {
    "#000000".to_string()
}

fn default_secondary_color() -> String {
    "#000000".to_string()
}

fn default_background_color() -> String {
    "#ffffff".to_string()
}

fn default_button_style() -> String {
    "rounded".to_string()
}

fn default_font_family() -> String {
    "system-ui, -apple-system, sans-serif".to_string()
}

fn default_show_footer() -> bool {
    true
}

fn default_link_spacing() -> String {
    "24px".to_string()
}

fn default_header_color() -> String {
    "#000000".to_string()
}

fn default_bio_color() -> String {
    "rgba(0, 0, 0, 0.7)".to_string()
}

fn default_link_title_color() -> String {
    "#000000".to_string()
}

fn default_link_description_color() -> String {
    "rgba(0, 0, 0, 0.6)".to_string()
}

impl Config {
    /// Loads configuration from a TOML file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML configuration file
    ///
    /// # Returns
    ///
    /// * `Ok(Config)` if the file was successfully parsed
    /// * `Err(anyhow::Error)` if the file couldn't be read or parsed
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref()).context("Failed to read config file")?;

        let config: Config = toml::from_str(&content).context("Failed to parse TOML config")?;

        Ok(config)
    }

    /// Validates the configuration
    ///
    /// Checks that:
    /// - Profile name is not empty
    /// - At least one link is defined
    /// - Dark mode setting is valid (auto, light, dark, or disable)
    /// - Link types are valid (block or space)
    /// - Block-type links have titles
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the configuration is valid
    /// * `Err(anyhow::Error)` with a descriptive error message if validation fails
    pub fn validate(&self) -> Result<()> {
        // Validate that required fields are not empty
        if self.profile.name.is_empty() {
            anyhow::bail!("Profile name cannot be empty");
        }

        if self.links.is_empty() {
            anyhow::bail!("At least one link must be defined");
        }

        // Validate dark mode
        let mode = self.dark_mode.mode.to_lowercase();
        if mode != "auto" && mode != "light" && mode != "dark" && mode != "disable" {
            anyhow::bail!(
                "Invalid dark_mode.mode '{}'. Must be 'auto', 'light', 'dark', or 'disable'",
                self.dark_mode.mode
            );
        }

        // Validate links
        for (idx, link) in self.links.iter().enumerate() {
            // Validate link type
            let link_type = link.link_type.to_lowercase();
            if link_type != "block" && link_type != "space" {
                let default_identifier = format!("index {}", idx);
                let link_identifier = link.title.as_deref().unwrap_or(&default_identifier);
                anyhow::bail!(
                    "Invalid link_type '{}' for link '{}'. Must be 'block' or 'space'",
                    link.link_type,
                    link_identifier
                );
            }

            // For block type, title is required
            if link_type == "block"
                && (link.title.is_none()
                    || link.title.as_ref().map(|t| t.is_empty()).unwrap_or(true))
            {
                anyhow::bail!(
                    "Link title cannot be empty for block type (link at index {})",
                    idx
                );
            }

            // For space type, height should be specified
            if link_type == "space" && link.height.is_none() {
                let default_identifier = format!("index {}", idx);
                let link_identifier = link.title.as_deref().unwrap_or(&default_identifier);
                eprintln!(
                    "Warning: Space type link '{}' has no height specified, using default",
                    link_identifier
                );
            }
        }

        Ok(())
    }
}
