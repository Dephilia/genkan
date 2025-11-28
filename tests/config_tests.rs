use genkan::config::Config;

#[test]
fn test_config_parsing() {
    let toml_str = r#"
        [profile]
        name = "Test User"
        bio = "Test bio"
        avatar = "test.png"

        [theme]
        name = "simple"

        [meta]
        title = "Test"
        description = "Test description"

        [[links]]
        title = "Test Link"
        url = "https://example.com"
    "#;

    let config: Config = toml::from_str(toml_str).unwrap();
    assert_eq!(config.profile.name, "Test User");
    assert_eq!(config.links.len(), 1);
    assert_eq!(config.image.avatar_size, 512);
    assert_eq!(config.image.social_icon_size, 128);
    assert_eq!(config.image.link_icon_size, 128);
    assert_eq!(config.image.favicon_size, 64);
}
