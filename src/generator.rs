//! Site generation and asset processing
//!
//! This module handles the core generation logic for Genkan, including:
//! - Template rendering (HTML and CSS)
//! - Image downloading, compression, and embedding
//! - SVG color processing for dark mode compatibility
//! - QR code generation
//! - Theme file loading

use crate::config::Config;
use anyhow::{Context, Result};
use image::Luma;
use qrcode::QrCode;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use tera::{Context as TeraContext, Tera};

/// Main site generator
///
/// The Generator orchestrates the entire site generation process,
/// including loading themes, processing assets, and rendering templates.
pub struct Generator {
    config: Config,
    pub theme_path: PathBuf,
    output_path: PathBuf,
}

impl Generator {
    /// Creates a new Generator instance
    ///
    /// # Arguments
    ///
    /// * `config` - Validated configuration for the site
    /// * `theme_path` - Path to the theme directory
    /// * `output_path` - Path where the output HTML file will be written
    pub fn new(config: Config, theme_path: PathBuf, output_path: PathBuf) -> Self {
        Self {
            config,
            theme_path,
            output_path,
        }
    }

    /// Generates the static site
    ///
    /// This is the main entry point for site generation. It:
    /// 1. Validates the configuration
    /// 2. Loads and renders theme files
    /// 3. Processes all images (download, resize, embed as base64)
    /// 4. Generates QR codes if configured
    /// 5. Renders the final HTML with all assets embedded
    /// 6. Writes the output file
    ///
    /// # Returns
    ///
    /// * `Ok(())` if generation was successful
    /// * `Err(anyhow::Error)` if any step failed
    pub fn generate(&self) -> Result<()> {
        // Validate config
        self.config
            .validate()
            .context("Configuration validation failed")?;

        // Load theme files
        let html_template = self.load_theme_file("template.html")?;
        let css_template = self.load_theme_file("style.css")?;
        let js_content = self.load_theme_file("script.js").unwrap_or_default();

        // Create Tera instance for CSS (which has template variables)
        let mut tera = Tera::default();
        tera.add_raw_template("style.css", &css_template)
            .context("Failed to add CSS template")?;

        // Process profile assets (download and embed external images)
        let mut processed_profile = self.config.profile.clone();

        // Get target sizes from config
        let avatar_size = Some(self.config.image.avatar_size);
        let social_icon_size = Some(self.config.image.social_icon_size);
        let link_icon_size = Some(self.config.image.link_icon_size);

        // Process light mode avatar
        if !processed_profile.light.avatar.is_empty() {
            match self.process_icon(&processed_profile.light.avatar, avatar_size) {
                Ok(processed) => processed_profile.light.avatar = processed,
                Err(e) => eprintln!("Warning: Failed to process light mode avatar: {}", e),
            }
        }

        // Process dark mode avatar
        if !processed_profile.dark.avatar.is_empty() {
            match self.process_icon(&processed_profile.dark.avatar, avatar_size) {
                Ok(processed) => processed_profile.dark.avatar = processed,
                Err(e) => eprintln!("Warning: Failed to process dark mode avatar: {}", e),
            }
        }

        // Process social link icons
        for social_link in &mut processed_profile.social_links {
            if !social_link.icon.is_empty() {
                match self.process_icon(&social_link.icon, social_icon_size) {
                    Ok(processed) => social_link.icon = processed,
                    Err(e) => eprintln!("Warning: Failed to process social link icon: {}", e),
                }
            }
        }

        // Process regular link icons (download and embed external images)
        let mut processed_links = self.config.links.clone();
        for link in &mut processed_links {
            if let Some(ref icon) = link.icon
                && !icon.is_empty()
            {
                match self.process_icon(icon, link_icon_size) {
                    Ok(processed) => link.icon = Some(processed),
                    Err(e) => eprintln!("Warning: Failed to process link icon: {}", e),
                }
            }
        }

        // Resolve typography values with theme colors
        let resolved_header = self.config.theme.typography.resolve(
            &self.config.theme.typography.header,
            Some(&self.config.theme.light.header_color),
            Some(&self.config.theme.dark.header_color),
        );
        let resolved_bio = self.config.theme.typography.resolve(
            &self.config.theme.typography.bio,
            Some(&self.config.theme.light.bio_color),
            Some(&self.config.theme.dark.bio_color),
        );
        let resolved_link_title = self.config.theme.typography.resolve(
            &self.config.theme.typography.link_title,
            Some(&self.config.theme.light.link_title_color),
            Some(&self.config.theme.dark.link_title_color),
        );
        let resolved_link_description = self.config.theme.typography.resolve(
            &self.config.theme.typography.link_description,
            Some(&self.config.theme.light.link_description_color),
            Some(&self.config.theme.dark.link_description_color),
        );

        // Create context for CSS rendering
        let mut css_context = TeraContext::new();
        css_context.insert("theme", &self.config.theme);
        css_context.insert("profile", &processed_profile);
        css_context.insert("typography_header", &resolved_header);
        css_context.insert("typography_bio", &resolved_bio);
        css_context.insert("typography_link_title", &resolved_link_title);
        css_context.insert("typography_link_description", &resolved_link_description);

        // Render CSS with variables
        let rendered_css = tera
            .render("style.css", &css_context)
            .context("Failed to render CSS template")?;

        // Create Tera instance for HTML
        let mut html_tera = Tera::default();
        html_tera
            .add_raw_template("template.html", &html_template)
            .context("Failed to add HTML template")?;

        // Generate QR code if page_url is provided
        let qr_code_data = if let Some(ref page_url) = self.config.meta.page_url {
            if !page_url.is_empty() {
                Some(self.generate_qr_code(page_url)?)
            } else {
                None
            }
        } else {
            None
        };

        // Process favicon (convert local files to data URLs)
        let favicon_size = Some(self.config.image.favicon_size);
        let processed_favicon = self.process_favicon(favicon_size)?;

        // Create context for HTML rendering
        let mut html_context = TeraContext::new();
        html_context.insert("profile", &processed_profile);
        html_context.insert("theme", &self.config.theme);
        html_context.insert("dark_mode", &self.config.dark_mode);

        // Create a modified meta object with processed favicon
        let mut meta_with_favicon = self.config.meta.clone();
        if let Some(ref favicon_data) = processed_favicon {
            meta_with_favicon.favicon = Some(favicon_data.clone());
        }
        html_context.insert("meta", &meta_with_favicon);

        html_context.insert("links", &processed_links);
        html_context.insert("css", &rendered_css);
        html_context.insert("js", &js_content);
        if let Some(ref qr_data) = qr_code_data {
            html_context.insert("qr_code_data", qr_data);
        }

        // Render final HTML
        let rendered_html = html_tera
            .render("template.html", &html_context)
            .context("Failed to render HTML template")?;

        // Create output directory if it doesn't exist
        if let Some(parent) = self.output_path.parent() {
            fs::create_dir_all(parent).context("Failed to create output directory")?;
        }

        // Write output file
        fs::write(&self.output_path, rendered_html).context("Failed to write output file")?;

        println!("Generated page at: {}", self.output_path.display());
        Ok(())
    }

    fn load_theme_file(&self, filename: &str) -> Result<String> {
        let file_path = self.theme_path.join(filename);
        fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read theme file: {}", file_path.display()))
    }

    fn generate_qr_code(&self, url: &str) -> Result<String> {
        use image::{DynamicImage, ImageFormat};
        use std::io::Cursor;

        // Generate QR code
        let code = QrCode::new(url.as_bytes()).context("Failed to create QR code")?;

        // Render QR code as image
        let image = code
            .render::<Luma<u8>>()
            .min_dimensions(200, 200)
            .max_dimensions(200, 200)
            .build();

        // Convert to PNG and encode as base64
        let mut png_data = Vec::new();
        let dynamic_image = DynamicImage::ImageLuma8(image);
        dynamic_image
            .write_to(&mut Cursor::new(&mut png_data), ImageFormat::Png)
            .context("Failed to encode QR code as PNG")?;

        // Create base64 data URL
        let base64_data =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png_data);
        Ok(format!("data:image/png;base64,{}", base64_data))
    }

    fn resize_image(&self, image_data: &[u8], target_size: u32) -> Result<Vec<u8>> {
        use image::{ImageFormat, imageops::FilterType};
        use std::io::Cursor;

        // Load the image
        let img =
            image::load_from_memory(image_data).context("Failed to load image for resizing")?;

        // Get current dimensions
        let (width, height) = (img.width(), img.height());

        // If image is already smaller than target size, return original
        if width <= target_size && height <= target_size {
            return Ok(image_data.to_vec());
        }

        // Calculate new dimensions maintaining aspect ratio
        let (new_width, new_height) = if width > height {
            let ratio = target_size as f32 / width as f32;
            (target_size, (height as f32 * ratio) as u32)
        } else {
            let ratio = target_size as f32 / height as f32;
            ((width as f32 * ratio) as u32, target_size)
        };

        // Resize the image using Lanczos3 filter for high quality
        let resized = img.resize(new_width, new_height, FilterType::Lanczos3);

        // Encode to PNG format
        let mut output = Vec::new();
        resized
            .write_to(&mut Cursor::new(&mut output), ImageFormat::Png)
            .context("Failed to encode resized image")?;

        Ok(output)
    }

    fn process_svg_for_inline(&self, svg_data: &[u8]) -> Result<String> {
        // Convert SVG data to string
        let mut svg_content =
            String::from_utf8(svg_data.to_vec()).context("Failed to parse SVG as UTF-8")?;

        // Replace hardcoded colors with currentColor to inherit CSS color
        use regex::Regex;

        // Remove XML declaration if present
        let xml_decl_regex =
            Regex::new(r"<\?xml[^?]*\?>").context("Failed to compile XML declaration regex")?;
        svg_content = xml_decl_regex.replace_all(&svg_content, "").to_string();

        // Remove or clean up HTML comments
        let comment_regex =
            Regex::new(r"<!--[^>]*-->").context("Failed to compile comment regex")?;
        svg_content = comment_regex.replace_all(&svg_content, "").to_string();

        // Remove fixed width and height attributes from the SVG tag (let CSS control dimensions)
        let width_regex =
            Regex::new(r#"\s+width="[^"]*""#).context("Failed to compile width regex")?;
        svg_content = width_regex.replace_all(&svg_content, "").to_string();

        let height_regex =
            Regex::new(r#"\s+height="[^"]*""#).context("Failed to compile height regex")?;
        svg_content = height_regex.replace_all(&svg_content, "").to_string();

        // Replace fill="<color>" with fill="currentColor" (but preserve fill="none")
        let fill_regex = Regex::new(r#"fill="[^"]*""#).context("Failed to compile fill regex")?;
        svg_content = fill_regex
            .replace_all(&svg_content, |caps: &regex::Captures| {
                // caps.get(0) is guaranteed to exist in replace_all callback
                let matched = caps.get(0).map(|m| m.as_str()).unwrap_or("");
                if matched == r#"fill="none""# {
                    matched.to_string()
                } else {
                    r#"fill="currentColor""#.to_string()
                }
            })
            .to_string();

        // Replace stroke="<color>" with stroke="currentColor" (but preserve stroke="none")
        let stroke_regex =
            Regex::new(r#"stroke="[^"]*""#).context("Failed to compile stroke regex")?;
        svg_content = stroke_regex
            .replace_all(&svg_content, |caps: &regex::Captures| {
                // caps.get(0) is guaranteed to exist in replace_all callback
                let matched = caps.get(0).map(|m| m.as_str()).unwrap_or("");
                if matched == r#"stroke="none""# {
                    matched.to_string()
                } else {
                    r#"stroke="currentColor""#.to_string()
                }
            })
            .to_string();

        // Also handle style attributes with fill/stroke
        let style_fill_regex =
            Regex::new(r"fill:\s*([^;]+)").context("Failed to compile style fill regex")?;
        svg_content = style_fill_regex
            .replace_all(&svg_content, |caps: &regex::Captures| {
                // caps.get(1) should exist if the regex matched
                let color = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                if color.trim() == "none" {
                    format!("fill:{}", color)
                } else {
                    "fill:currentColor".to_string()
                }
            })
            .to_string();

        let style_stroke_regex =
            Regex::new(r"stroke:\s*([^;]+)").context("Failed to compile style stroke regex")?;
        svg_content = style_stroke_regex
            .replace_all(&svg_content, |caps: &regex::Captures| {
                // caps.get(1) should exist if the regex matched
                let color = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                if color.trim() == "none" {
                    format!("stroke:{}", color)
                } else {
                    "stroke:currentColor".to_string()
                }
            })
            .to_string();

        // Return with special marker to indicate this is inline SVG
        Ok(format!("__INLINE_SVG__{}", svg_content))
    }

    fn download_and_embed_image(&self, url: &str, target_size: Option<u32>) -> Result<String> {
        // Download the image
        let response = ureq::get(url)
            .set("User-Agent", "Mozilla/5.0 (compatible; Genkan/1.0)")
            .timeout(std::time::Duration::from_secs(10))
            .call()
            .with_context(|| format!("Failed to download image from: {}", url))?;

        // Read response body
        let mut image_data = Vec::new();
        response
            .into_reader()
            .read_to_end(&mut image_data)
            .with_context(|| format!("Failed to read image data from: {}", url))?;

        // Check if it's an SVG (don't resize SVGs)
        let is_svg = url.ends_with(".svg")
            || url.contains(".svg?")
            || (image_data.len() > 5 && &image_data[0..5] == b"<?xml")
            || (image_data.len() > 4 && &image_data[0..4] == b"<svg");

        // If it's an SVG, process it for inline rendering
        if is_svg {
            return self.process_svg_for_inline(&image_data);
        }

        // Resize if target_size is specified
        let final_data = if let Some(size) = target_size {
            match self.resize_image(&image_data, size) {
                Ok(resized) => {
                    println!(
                        "Compressed image from {} to {} bytes (target size: {}px)",
                        image_data.len(),
                        resized.len(),
                        size
                    );
                    resized
                }
                Err(e) => {
                    eprintln!("Warning: Failed to resize image: {}. Using original.", e);
                    image_data
                }
            }
        } else {
            image_data
        };

        // Determine MIME type - use PNG for resized images
        let mime_type = if target_size.is_some() {
            "image/png"
        } else if url.ends_with(".jpg")
            || url.ends_with(".jpeg")
            || url.contains(".jpg?")
            || url.contains(".jpeg?")
        {
            "image/jpeg"
        } else if url.ends_with(".gif") || url.contains(".gif?") {
            "image/gif"
        } else if url.ends_with(".webp") || url.contains(".webp?") {
            "image/webp"
        } else if url.ends_with(".ico") || url.contains(".ico?") {
            "image/x-icon"
        } else {
            // Default to PNG for .png files and unknown types
            "image/png"
        };

        // Encode as base64
        let base64_data =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &final_data);
        Ok(format!("data:{};base64,{}", mime_type, base64_data))
    }

    fn process_icon(&self, icon: &str, target_size: Option<u32>) -> Result<String> {
        // If it's already a data URL, return as-is
        if icon.starts_with("data:") {
            return Ok(icon.to_string());
        }

        // If it's an external URL, download and embed it
        if icon.starts_with("http://") || icon.starts_with("https://") || icon.starts_with("//") {
            match self.download_and_embed_image(icon, target_size) {
                Ok(embedded) => {
                    println!("Embedded external icon: {}", icon);
                    return Ok(embedded);
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to download icon '{}': {}. Using original URL.",
                        icon, e
                    );
                    return Ok(icon.to_string());
                }
            }
        }

        // If it's a local file path, read and convert to data URL
        let file_path = PathBuf::from(icon);
        if file_path.exists() {
            let file_data = fs::read(&file_path)
                .with_context(|| format!("Failed to read icon file: {}", file_path.display()))?;

            let is_svg = matches!(file_path.extension().and_then(|e| e.to_str()), Some("svg"));

            // If it's an SVG, process it for inline rendering
            if is_svg {
                return self.process_svg_for_inline(&file_data);
            }

            // Resize local files too if target_size is specified
            let final_data = if let Some(size) = target_size {
                match self.resize_image(&file_data, size) {
                    Ok(resized) => {
                        println!(
                            "Compressed local icon from {} to {} bytes (target size: {}px)",
                            file_data.len(),
                            resized.len(),
                            size
                        );
                        resized
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to resize icon: {}. Using original.", e);
                        file_data
                    }
                }
            } else {
                file_data
            };

            let mime_type = if target_size.is_some() {
                "image/png"
            } else {
                match file_path.extension().and_then(|e| e.to_str()) {
                    Some("png") => "image/png",
                    Some("jpg") | Some("jpeg") => "image/jpeg",
                    Some("gif") => "image/gif",
                    Some("webp") => "image/webp",
                    Some("ico") => "image/x-icon",
                    _ => "image/png",
                }
            };

            let base64_data =
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &final_data);
            return Ok(format!("data:{};base64,{}", mime_type, base64_data));
        }

        // Not a URL or file path - probably an emoji or text, return as-is
        Ok(icon.to_string())
    }

    fn process_favicon(&self, target_size: Option<u32>) -> Result<Option<String>> {
        let favicon = match &self.config.meta.favicon {
            Some(f) if !f.is_empty() => f,
            _ => return Ok(None),
        };

        // If it's already a data URL, return as-is
        if favicon.starts_with("data:") {
            return Ok(Some(favicon.clone()));
        }

        // If it's an external URL, download and embed it
        if favicon.starts_with("http://")
            || favicon.starts_with("https://")
            || favicon.starts_with("//")
        {
            match self.download_and_embed_image(favicon, target_size) {
                Ok(embedded) => {
                    println!("Embedded favicon: {}", favicon);
                    return Ok(Some(embedded));
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to download favicon '{}': {}. Using original URL.",
                        favicon, e
                    );
                    return Ok(Some(favicon.clone()));
                }
            }
        }

        // It's a local file path - read and convert to data URL
        let file_path = PathBuf::from(favicon);

        // Check if file exists
        if !file_path.exists() {
            eprintln!("Warning: Favicon file not found: {}", favicon);
            return Ok(None);
        }

        // Read file
        let file_data = fs::read(&file_path)
            .with_context(|| format!("Failed to read favicon file: {}", file_path.display()))?;

        let is_svg = matches!(file_path.extension().and_then(|e| e.to_str()), Some("svg"));
        let is_ico = matches!(file_path.extension().and_then(|e| e.to_str()), Some("ico"));

        // Resize if target_size is specified and it's not SVG or ICO
        let final_data = if let Some(size) = target_size {
            if is_svg || is_ico {
                file_data
            } else {
                match self.resize_image(&file_data, size) {
                    Ok(resized) => {
                        println!(
                            "Compressed favicon from {} to {} bytes (target size: {}px)",
                            file_data.len(),
                            resized.len(),
                            size
                        );
                        resized
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to resize favicon: {}. Using original.", e);
                        file_data
                    }
                }
            }
        } else {
            file_data
        };

        // Determine MIME type from extension
        let mime_type = if target_size.is_some() && !is_svg && !is_ico {
            "image/png"
        } else {
            match file_path.extension().and_then(|e| e.to_str()) {
                Some("ico") => "image/x-icon",
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("gif") => "image/gif",
                Some("svg") => "image/svg+xml",
                Some("webp") => "image/webp",
                _ => {
                    eprintln!("Warning: Unknown favicon file type, defaulting to image/x-icon");
                    "image/x-icon"
                }
            }
        };

        // Encode as base64
        let base64_data =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &final_data);
        Ok(Some(format!("data:{};base64,{}", mime_type, base64_data)))
    }
}

/// Finds the path to a theme directory
///
/// Searches for the theme in multiple locations:
/// 1. `themes/{theme_name}`
/// 2. `./themes/{theme_name}`
/// 3. `../themes/{theme_name}`
///
/// # Arguments
///
/// * `theme_name` - Name of the theme to find
///
/// # Returns
///
/// * `Ok(PathBuf)` with the path to the theme directory
/// * `Err(anyhow::Error)` if the theme was not found
pub fn find_theme_path(theme_name: &str) -> Result<PathBuf> {
    // Try multiple locations for theme directory
    let possible_paths = vec![
        PathBuf::from(format!("themes/{}", theme_name)),
        PathBuf::from(format!("./themes/{}", theme_name)),
        PathBuf::from(format!("../themes/{}", theme_name)),
    ];

    for path in possible_paths {
        if path.exists() && path.is_dir() {
            return Ok(path);
        }
    }

    anyhow::bail!(
        "Theme '{}' not found. Please ensure the theme directory exists in the themes folder.",
        theme_name
    )
}
