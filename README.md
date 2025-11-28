# Genkan (玄関)

[![Crates.io](https://img.shields.io/crates/v/genkan.svg)](https://crates.io/crates/genkan)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/dephilia/genkan/ci.yml?branch=main)](https://github.com/dephilia/genkan/actions)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

> A fast, customizable static site generator for creating beautiful Linktree-like pages

Genkan (Japanese for "entrance" or "foyer") is a lightweight static site generator that creates beautiful, responsive link pages similar to Linktree, but with the power of self-hosting and complete customization.

## Features

- **Fast & Lightweight**: Built in Rust for blazing-fast generation
- **Fully Customizable**: Complete control over themes, colors, and layout
- **Responsive Design**: Works perfectly on desktop and mobile devices
- **Simple Configuration**: Easy-to-use TOML configuration
- **Self-Hosted**: Generate static HTML - host anywhere
- **Theme System**: Create and share custom themes
- **No Dependencies**: Output is pure HTML/CSS/JS - no runtime dependencies
- **Dark Mode Support**: Auto, light, dark, or disabled modes with separate color schemes
- **Image Compression**: Automatic download, resize, and embedding of all images for faster loading
- **Share Button**: Built-in share button with QR code generation
- **Background Images**: Support for custom background images and gradients
- **Customizable Footer**: Option to hide or show the "Made with Genkan" footer

## Installation

### From crates.io (Recommended)

```bash
cargo install genkan
```

### From Source

#### Prerequisites

- Rust 1.70 or higher (install from [rustup.rs](https://rustup.rs/))

```bash
# Clone the repository
git clone https://github.com/dephilia/genkan.git
cd genkan

# Build the project
cargo build --release

# The binary will be in target/release/genkan
# Optionally, install it globally
cargo install --path .
```

## Quick Start

### 1. Initialize a New Project

```bash
genkan init my-links
cd my-links
```

This creates:
```
my-links/
├── config.toml    # Your configuration file
├── themes/        # Theme directory
└── output/        # Generated output
```

### 2. Edit Your Configuration

Open `config.toml` and customize it with your information:

```toml
[profile]
name = "Your Name"
bio = "Welcome to my link page! 👋"
avatar = "https://your-avatar-url.com/image.jpg"

[[links]]
title = "My Website"
url = "https://example.com"
icon = "🌐"

[[links]]
title = "Twitter"
url = "https://twitter.com/yourhandle"
icon = "🐦"
```

### 3. Generate Your Page

```bash
genkan build
```

Your page is now available at `output/index.html`!

### 4. Open in Browser

Simply open `output/index.html` in your browser, or deploy it to any static hosting service (GitHub Pages, Netlify, Vercel, etc.).

## Project Structure

After initialization, your project will have this structure:

```
genkan/
├── Cargo.toml              # Rust project configuration
├── README.md               # Documentation
├── LICENSE                 # MIT License
├── config.toml             # Your configuration file
├── src/
│   ├── main.rs             # CLI entry point
│   ├── config.rs           # Configuration parser
│   └── generator.rs        # Page generator
├── themes/
│   └── simple/
│       ├── template.html   # HTML template
│       ├── style.css       # Responsive CSS
│       └── script.js       # Optional JavaScript
└── output/
    └── index.html          # Generated page
```

## Customization Quick Reference

Need to make quick changes? Here are the most common customizations:

### Change Colors
Edit `config.toml`:
```toml
[theme]
primary_color = "#667eea"
secondary_color = "#2d3748"
background_color = "#f7fafc"
```

### Add a Link
Add to `config.toml`:
```toml
[[links]]
title = "My Link"
url = "https://example.com"
icon = "🔗"  # emoji, URL, or omit for default
description = "Optional subtitle"
```

### Change Button Style
Edit `config.toml`:
```toml
[theme]
button_style = "pill"  # options: rounded, pill, square
```

### Add Background Gradient or Image
Edit `config.toml`:
```toml
[profile]
# Option 1: Gradient or solid color
background = "linear-gradient(135deg, #667eea 0%, #764ba2 100%)"

# Option 2: Background image (takes priority)
background_image = "https://example.com/bg.jpg"
```

### Hide Footer
Edit `config.toml`:
```toml
[meta]
show_footer = false  # Hide "Made with Genkan" footer
```

### Add Custom CSS
Edit `config.toml`:
```toml
[meta]
custom_css = """
.link-button {
    background: linear-gradient(90deg, #667eea, #764ba2);
}
"""
```

## Configuration Guide

### Profile Section

The `[profile]` section controls your personal information:

```toml
[profile]
name = "Your Name"                    # Your display name
bio = "Welcome to my link page! 👋"   # Short bio or tagline
avatar = "https://example.com/me.jpg" # Profile picture (URL or local path)
background = "linear-gradient(135deg, #667eea 0%, #764ba2 100%)"  # Optional background
```

#### Avatar Options:
- **URL**: `avatar = "https://example.com/avatar.jpg"`
- **Local file**: `avatar = "./images/me.jpg"` (relative to config.toml)
- If omitted, a default icon will be shown

#### Background Options:
- **Solid color**: `background = "#ff6b6b"`
- **Gradient**: `background = "linear-gradient(135deg, #667eea 0%, #764ba2 100%)"`
- **Background image (URL)**: `background_image = "https://example.com/bg.jpg"`
- **Background image (local)**: `background_image = "./images/background.jpg"`
- Note: `background_image` takes priority over `background` if both are set

### Theme Section

The `[theme]` section controls the visual appearance:

```toml
[theme]
name = "simple"                       # Theme name
primary_color = "#000000"             # Primary color for buttons and accents
secondary_color = "#000000"           # Secondary color for accents
background_color = "#ffffff"          # Page background
button_style = "rounded"              # Button shape: rounded, pill, square
font_family = "system-ui, -apple-system, sans-serif"  # Font
link_spacing = "32px"                 # Space between links

# Legacy color domains (deprecated - use typography system instead)
header_color = "#000000"              # Profile name/header color
bio_color = "rgba(0, 0, 0, 0.7)"      # Bio/description text color
link_title_color = "#000000"          # Link title text color
link_description_color = "rgba(0, 0, 0, 0.6)"  # Link description text color
```

#### Color Domains:
- **A. Header** (`header_color`): Profile name at the top
- **B. Bio/Description** (`bio_color`): Bio text under profile name
- **C. Primary Color** (`primary_color`): Buttons, accents, and interactive elements
- **D. Link Title** (`link_title_color`): Main text in link blocks
- **E. Link Description** (`link_description_color`): Subtitle/description in link blocks

#### Button Styles:
- `rounded`: Slightly rounded corners (12px radius)
- `pill`: Fully rounded ends (50px radius)
- `square`: Sharp corners (4px radius)

### Typography System (Advanced)

The `[theme.typography]` section provides granular control over text styling. Each text element (header, bio, link_title, link_description) can have its own size, font, weight, style, and color settings.

```toml
[theme.typography.default]
size = "16px"
font = "system-ui, -apple-system, sans-serif"
weight = "normal"
style = "normal"
color = "#000000"

[theme.typography.header]
size = "2rem"
# font = "" # empty or omit to use default
weight = "700"
style = "normal"
color = "#000000"

[theme.typography.bio]
size = "1.1rem"
weight = "normal"
style = "normal"
color = "rgba(0, 0, 0, 0.7)"

[theme.typography.link_title]
size = "1.1rem"
weight = "600"
style = "normal"
color = "#000000"

[theme.typography.link_description]
size = "0.9rem"
weight = "normal"
style = "italic"  # Can be "normal" or "italic"
color = "rgba(0, 0, 0, 0.6)"
```

**Typography Properties:**
- `size`: Font size (e.g., "16px", "1.2rem", "1em")
- `font`: Font family (e.g., "Arial, sans-serif")
- `weight`: Font weight (e.g., "normal", "bold", "600", "700")
- `style`: Font style ("normal" or "italic")
- `color`: Text color (any valid CSS color)

**Fallback Behavior:**
- If a property is not specified for an element, it falls back to the `default` values
- If `default` is not specified, it uses built-in defaults
- For backward compatibility, the old color fields (`header_color`, `bio_color`, etc.) are still supported

### Dark Mode

Genkan supports automatic dark mode with separate color schemes for light and dark themes:

```toml
[dark_mode]
mode = "auto"  # Options: "auto", "light", "dark", "disable"

# Light mode colors
[theme.light]
primary_color = "#000000"
secondary_color = "#000000"
background_color = "#ffffff"
header_color = "#000000"
bio_color = "rgba(0, 0, 0, 0.7)"
link_title_color = "#000000"
link_description_color = "rgba(0, 0, 0, 0.6)"

# Dark mode colors
[theme.dark]
primary_color = "#ffffff"
secondary_color = "#ffffff"
background_color = "#1a1a1a"
header_color = "#ffffff"
bio_color = "rgba(255, 255, 255, 0.7)"
link_title_color = "#ffffff"
link_description_color = "rgba(255, 255, 255, 0.6)"

# Optional: Separate avatars for light and dark mode
[profile.light]
avatar = "https://example.com/avatar-light.png"

[profile.dark]
avatar = "https://example.com/avatar-dark.png"
```

**Dark Mode Options:**
- `auto`: Automatically switch based on system preference (default)
- `light`: Force light mode only
- `dark`: Force dark mode only
- `disable`: Disable dark mode support (uses light colors only)

**Typography in Dark Mode:**

You can also specify different colors for dark mode in the typography system:

```toml
[theme.typography.header]
color = "#000000"              # Light mode color
color_dark = "#ffffff"         # Dark mode color
```

### Meta Section

The `[meta]` section configures page metadata and extras:

```toml
[meta]
title = "My Links"                    # Page title (appears in browser tab)
description = "All my links"          # Page description (for SEO)
favicon = "https://example.com/favicon.ico"  # Optional favicon (URL or local path)
custom_css = ""                       # Optional custom CSS
analytics = ""                        # Optional analytics code
show_footer = false                   # Hide "Made with Genkan" footer (default: true)
```

#### Favicon Options:
- **URL**: `favicon = "https://example.com/favicon.ico"`
- **Local file**: `favicon = "./images/favicon.png"` (relative to config.toml)
- **Supported formats**: .ico, .png, .jpg, .svg, .gif, .webp
- Local files are automatically embedded as base64 data URLs

#### Adding Custom CSS:

```toml
[meta]
custom_css = """
.link-button:hover {
    background: linear-gradient(90deg, #667eea, #764ba2);
    color: white;
}
"""
```

#### Adding Analytics:

```toml
[meta]
analytics = """
<script async src="https://www.googletagmanager.com/gtag/js?id=GA_MEASUREMENT_ID"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'GA_MEASUREMENT_ID');
</script>
"""
```

### Image Section

The `[image]` section controls automatic image compression and resizing for faster page loads:

```toml
[image]
avatar_size = 512           # Target size for profile avatars (default: 512px)
social_icon_size = 128      # Target size for social link icons (default: 128px)
link_icon_size = 128        # Target size for link icons (default: 128px)
favicon_size = 64           # Target size for favicon (default: 64px)
```

#### How Image Compression Works:

1. **Automatic Download**: All external image URLs are downloaded during generation
2. **Smart Resizing**: Images are resized to target dimensions while maintaining aspect ratio
3. **High-Quality Compression**: Uses Lanczos3 filter for excellent visual quality
4. **Base64 Embedding**: All images are embedded as base64 data URLs in the HTML
5. **Self-Contained Output**: The generated HTML file has no external dependencies

#### Special Cases:

- **SVG images**: Never resized to preserve vector quality
- **ICO favicons**: Not resized (kept as-is for compatibility)
- **Small images**: If already smaller than target size, kept at original dimensions
- **Emojis**: Text emojis (🌐, 📧, etc.) are passed through without processing

#### Performance Benefits:

Real-world example from testing:
- X logo icon: Reduced from 98KB to 6.5KB (93% reduction!)
- Bluesky logo: Reduced from 41KB to 8KB (81% reduction)
- Misskey icon: Reduced from 23KB to 9KB (60% reduction)

The compression happens automatically during the build process, with progress logged to the console.

#### Configuration Tips:

- **For profile avatars**: 512px provides excellent quality for displays
- **For link icons**: 128px is optimal for link button icons
- **For social icons**: 128px works well for small social media icons
- **For favicons**: 64px is standard for modern favicons
- Adjust sizes based on your design needs - larger values = better quality but bigger file size

### Links Section

Add as many links as you want using `[[links]]`:

```toml
[[links]]
title = "My Website"                  # Link title (required)
url = "https://example.com"           # Link URL (optional - omit for non-clickable)
icon = "🌐"                           # Icon (optional - omit for text-only)
description = "Check out my site"     # Subtitle (optional)
link_type = "block"                   # Type: "block" or "space" (default: "block")
height = "40px"                       # Height for spacers (only for link_type = "space")
```

#### Link Types:

**Block Type** (default):
- Standard clickable or non-clickable link box
- Omit `url` to make it non-clickable (displays as text box)
- Omit `icon` for text-only display (no icon shown)

```toml
# Standard clickable link
[[links]]
title = "My Website"
url = "https://example.com"
icon = "🌐"
link_type = "block"

# Non-clickable text block (no URL)
[[links]]
title = "Important Notice"
description = "This is just informational text"
link_type = "block"

# Text-only clickable link (no icon)
[[links]]
title = "Simple Link"
url = "https://example.com"
link_type = "block"
```

**Space Type**:
- Creates vertical spacing between links
- Use `height` to control the space (e.g., "20px", "40px")

```toml
[[links]]
title = ""
link_type = "space"
height = "30px"
```

#### Icon Options:

1. **Emoji**: Use any emoji
   ```toml
   icon = "🌐"
   icon = "💼"
   icon = "📧"
   ```

2. **Image URL**: Use an icon from the web
   ```toml
   icon = "https://cdn.simpleicons.org/github/000000"
   icon = "https://example.com/my-icon.png"
   ```

3. **No icon**: Omit the icon field for a default link icon
   ```toml
   [[links]]
   title = "My Link"
   url = "https://example.com"
   # No icon specified - will show default 🔗
   ```

#### Popular Icon Services:
- [Simple Icons](https://simpleicons.org/): `https://cdn.simpleicons.org/[brand]/[color]`
  - GitHub: `https://cdn.simpleicons.org/github/000000`
  - Twitter: `https://cdn.simpleicons.org/twitter/1DA1F2`
  - LinkedIn: `https://cdn.simpleicons.org/linkedin/0A66C2`

### Complete Example

Here's a complete configuration example:

```toml
[profile]
name = "Jane Doe"
bio = "Designer, Developer & Creator ✨"
avatar = "https://avatars.githubusercontent.com/u/123456"
background = "linear-gradient(135deg, #667eea 0%, #764ba2 100%)"

[theme]
name = "simple"
primary_color = "#667eea"
secondary_color = "#2d3748"
background_color = "#ffffff"
button_style = "pill"
font_family = "Inter, system-ui, sans-serif"

[meta]
title = "Jane Doe | Links"
description = "All my important links in one place"
favicon = "https://example.com/favicon.ico"

[image]
avatar_size = 512
social_icon_size = 128
link_icon_size = 128
favicon_size = 64

[[links]]
title = "Portfolio"
url = "https://janedoe.com"
icon = "🎨"
description = "View my work and projects"

[[links]]
title = "GitHub"
url = "https://github.com/janedoe"
icon = "https://cdn.simpleicons.org/github/000000"

[[links]]
title = "Twitter"
url = "https://twitter.com/janedoe"
icon = "https://cdn.simpleicons.org/twitter/1DA1F2"

[[links]]
title = "Email"
url = "mailto:hello@janedoe.com"
icon = "✉️"
description = "Get in touch"
```

## CLI Commands

### Build

Generate your site from the configuration:

```bash
genkan build                          # Use default config.toml
genkan build -c custom.toml           # Use custom config file
genkan build -o dist                  # Output to custom directory
```

### Init

Initialize a new project:

```bash
genkan init                           # Initialize in current directory
genkan init my-project                # Initialize in new directory
```

### Validate

Validate your configuration without building:

```bash
genkan validate                       # Validate config.toml
genkan validate -c custom.toml        # Validate custom config
```

## Built-in Features

### Share Button with QR Code

Every generated page includes a share button (top right corner) that allows visitors to:
- **Generate QR Code**: Automatically creates a QR code for the current page URL
- **Copy Link**: Quick copy-to-clipboard functionality
- **Mobile Friendly**: Works seamlessly on desktop and mobile devices

The share button appears as a floating button in the top-right corner and opens a modal with:
1. A scannable QR code (great for sharing in person)
2. The page URL in a text field
3. A copy button for quick clipboard access

No configuration needed - it works out of the box on every generated page!

### Customizable Footer

Control whether to show the "Made with Genkan" footer:

```toml
[meta]
show_footer = false  # Hide the footer (default: true)
```

### Background Images

Use custom background images for your page:

```toml
[profile]
# Option 1: Use a gradient or solid color
background = "linear-gradient(135deg, #667eea 0%, #764ba2 100%)"

# Option 2: Use a background image (takes priority)
background_image = "https://example.com/background.jpg"
```

Background images are automatically set to:
- Cover the entire viewport
- Stay fixed during scrolling
- Center positioned
- No repeat

## Deployment

Since Genkan generates static HTML, you can deploy anywhere:

### GitHub Pages

```bash
# Generate your site
genkan build -o docs

# Commit and push
git add docs/
git commit -m "Update link page"
git push

# Enable GitHub Pages in repository settings (source: docs folder)
```

### Netlify

```bash
# Generate your site
genkan build

# Drag and drop the output folder to Netlify
# Or connect your Git repository
```

### Vercel

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy
genkan build
cd output
vercel
```

### Traditional Web Hosting

Simply upload the contents of the `output/` directory to your web server via FTP/SFTP.

## Creating Custom Themes

Genkan supports custom themes! Each theme consists of three files:

```
themes/
└── my-theme/
    ├── template.html    # HTML structure with placeholders
    ├── style.css        # Styles (supports template variables)
    └── script.js        # Optional JavaScript
```

### Template Variables

**template.html** supports these Tera template variables:

```html
{{ profile.name }}         <!-- Profile name -->
{{ profile.bio }}          <!-- Profile bio -->
{{ profile.avatar }}       <!-- Avatar URL -->
{{ profile.background }}   <!-- Background style -->

{{ theme.primary_color }}  <!-- Primary color -->
{{ theme.text_color }}     <!-- Text color -->
{{ theme.button_style }}   <!-- Button style -->

{{ meta.title }}           <!-- Page title -->
{{ meta.description }}     <!-- Page description -->

{% for link in links %}
{{ link.title }}           <!-- Link title -->
{{ link.url }}             <!-- Link URL -->
{{ link.icon }}            <!-- Link icon -->
{{ link.description }}     <!-- Link description -->
{% endfor %}
```

**style.css** supports template variables too:

```css
:root {
    --primary-color: {{ theme.primary_color }};
    --text-color: {{ theme.text_color }};
}
```

See `themes/simple/` for a complete example.

## Troubleshooting

### "Theme not found" Error

Make sure your theme exists in the `themes/` directory:
```
themes/
└── simple/           # Theme name must match config
    ├── template.html
    ├── style.css
    └── script.js
```

### "Failed to parse TOML" Error

Check your config.toml syntax:
- Strings must be in quotes: `title = "My Link"`
- Each `[[links]]` section must have `title` and `url`
- Colors must be valid CSS: `#000000` or `rgb(0,0,0)`

### Links Not Opening

Make sure URLs include the protocol:
- ✅ `url = "https://example.com"`
- ❌ `url = "example.com"`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Inspired by Linktree and similar services
- Built with Rust, Tera, and modern web standards
- Name inspired by Japanese architecture (genkan = entrance)

---

Made with ❤️ using Genkan
