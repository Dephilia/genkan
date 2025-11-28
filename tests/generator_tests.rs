use genkan::config::{
    Config, DarkMode, ImageSettings, Link, Meta, Profile, ProfileAssets, Theme, ThemeColors,
    Typography,
};
use genkan::generator::Generator;
use std::path::PathBuf;

#[test]
fn test_generator_creation() {
    let config = Config {
        profile: Profile {
            name: "Test".to_string(),
            bio: "Bio".to_string(),
            social_links: vec![],
            light: ProfileAssets {
                avatar: "avatar.png".to_string(),
                background: None,
                background_image: None,
            },
            dark: ProfileAssets::default(),
        },
        theme: Theme {
            name: "simple".to_string(),
            button_style: "rounded".to_string(),
            font_family: "sans-serif".to_string(),
            link_spacing: "24px".to_string(),
            typography: Typography::default(),
            light: ThemeColors::default(),
            dark: ThemeColors::default(),
        },
        meta: Meta {
            title: "Test".to_string(),
            description: "Test".to_string(),
            page_url: None,
            favicon: None,
            custom_css: None,
            analytics: None,
            show_footer: true,
            share_title: None,
        },
        links: vec![Link {
            title: Some("Test".to_string()),
            url: Some("https://example.com".to_string()),
            icon: None,
            description: None,
            link_type: "block".to_string(),
            height: None,
        }],
        dark_mode: DarkMode::default(),
        image: ImageSettings::default(),
    };

    let generator = Generator::new(
        config,
        PathBuf::from("themes/simple"),
        PathBuf::from("output/index.html"),
    );

    assert_eq!(generator.theme_path, PathBuf::from("themes/simple"));
}
