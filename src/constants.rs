use colored::Colorize;

pub fn get_logo() -> String {
    format!(
        r#"
    ███╗   ██╗{}
    ████╗  ██║{}
    ██╔██╗ ██║{}
    ██║╚██╗██║{}
    ██║ ╚████║{}
    ╚═╝  ╚═══╝{}"#,
        "███████╗".blue(),
        "╚════██║".blue(),
        "    ██╔╝".blue(),
        "   ██╔╝".blue(),
        "   ██║".blue(),
        "   ╚═╝".blue()
    )
}

pub fn home_banner() -> String {
    format!(
        r#"
        {}
    Cli nseven - All tools cli
    Home - Main command list"#,
        get_logo()
    )
}

pub fn dc_banner() -> String {
    format!(
        r#"
        {}
    Docker compose - List of commands to execute in a container"#,
        get_logo()
    )
}

pub fn version_banner() -> String {
    format!(
        r#"{}
    Version cli nseven"#,
        get_logo()
    )
}
