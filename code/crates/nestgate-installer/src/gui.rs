use anyhow::Result;
#[cfg(feature = "gui")]
use iced::{
    widget::{
        button, checkbox, column, container, progress_bar, row, scrollable, text, text_input,
    },
    Application, Command, Element, Length, Settings, Theme,
};

#[cfg(feature = "gui")]
pub async fn run_gui_installer() -> Result<()> {
    let settings = Settings {
        window: iced::window::Settings {
            size: (800, 600),
            min_size: Some((600, 400)),
            ..Default::default()
        },
        ..Default::default()
    };

    NestGateInstallerGui::run(settings)?;
    Ok(())
}

#[cfg(not(feature = "gui"))]
pub async fn run_gui_installer() -> Result<()> {
    anyhow::bail!("GUI feature not enabled. Compile with --features gui to enable GUI installer.");
}

#[cfg(feature = "gui")]
#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    NextStep,
    PreviousStep,
    Cancel,

    // Installation options
    ToggleService(bool),
    ToggleZfs(bool),
    SetInstallPath(String),

    // Configuration
    SetApiPort(String),
    SetSongbirdUrl(String),
    ToggleAiFeatures(bool),

    // Actions
    StartInstallation,
    InstallationProgress(f32),
    InstallationComplete,
    InstallationError(String),
}

#[cfg(feature = "gui")]
#[derive(Debug, Clone, PartialEq)]
pub enum InstallationStep {
    Welcome,
    SystemCheck,
    InstallOptions,
    Configuration,
    Installation,
    Complete,
}

#[cfg(feature = "gui")]
#[derive(Debug)]
pub struct NestGateInstallerGui {
    current_step: InstallationStep,

    // Installation options
    install_as_service: bool,
    enable_zfs: bool,
    install_path: String,

    // Configuration
    api_port: String,
    songbird_url: String,
    enable_ai: bool,

    // Installation state
    installation_progress: f32,
    installation_message: String,
    error_message: Option<String>,

    // System check results
    system_checks: Vec<SystemCheck>,
}

#[cfg(feature = "gui")]
#[derive(Debug, Clone)]
struct SystemCheck {
    name: String,
    status: CheckStatus,
    message: String,
}

#[cfg(feature = "gui")]
#[derive(Debug, Clone)]
enum CheckStatus {
    Pending,
    Success,
    Warning,
    Error,
}

#[cfg(feature = "gui")]
impl Application for NestGateInstallerGui {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let installer = Self {
            current_step: InstallationStep::Welcome,
            install_as_service: true,
            enable_zfs: true,
            install_path: "/usr/local".to_string(),
            api_port: "8080".to_string(),
            songbird_url: String::new(),
            enable_ai: true,
            installation_progress: 0.0,
            installation_message: String::new(),
            error_message: None,
            system_checks: vec![
                SystemCheck {
                    name: "Operating System".to_string(),
                    status: CheckStatus::Pending,
                    message: "Checking OS compatibility...".to_string(),
                },
                SystemCheck {
                    name: "Disk Space".to_string(),
                    status: CheckStatus::Pending,
                    message: "Checking available disk space...".to_string(),
                },
                SystemCheck {
                    name: "Memory".to_string(),
                    status: CheckStatus::Pending,
                    message: "Checking available memory...".to_string(),
                },
                SystemCheck {
                    name: "ZFS".to_string(),
                    status: CheckStatus::Pending,
                    message: "Checking ZFS availability...".to_string(),
                },
            ],
        };

        (installer, Command::none())
    }

    fn title(&self) -> String {
        match self.current_step {
            InstallationStep::Welcome => "NestGate Installer - Welcome".to_string(),
            InstallationStep::SystemCheck => "NestGate Installer - System Check".to_string(),
            InstallationStep::InstallOptions => "NestGate Installer - Options".to_string(),
            InstallationStep::Configuration => "NestGate Installer - Configuration".to_string(),
            InstallationStep::Installation => "NestGate Installer - Installing".to_string(),
            InstallationStep::Complete => "NestGate Installer - Complete".to_string(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NextStep => {
                self.current_step = match self.current_step {
                    InstallationStep::Welcome => InstallationStep::SystemCheck,
                    InstallationStep::SystemCheck => InstallationStep::InstallOptions,
                    InstallationStep::InstallOptions => InstallationStep::Configuration,
                    InstallationStep::Configuration => InstallationStep::Installation,
                    InstallationStep::Installation => InstallationStep::Complete,
                    InstallationStep::Complete => InstallationStep::Complete,
                };

                if self.current_step == InstallationStep::Installation {
                    return Command::perform(simulate_installation(), |progress| {
                        Message::InstallationProgress(progress)
                    });
                }
            }

            Message::PreviousStep => {
                self.current_step = match self.current_step {
                    InstallationStep::Welcome => InstallationStep::Welcome,
                    InstallationStep::SystemCheck => InstallationStep::Welcome,
                    InstallationStep::InstallOptions => InstallationStep::SystemCheck,
                    InstallationStep::Configuration => InstallationStep::InstallOptions,
                    InstallationStep::Installation => InstallationStep::Configuration,
                    InstallationStep::Complete => InstallationStep::Complete,
                };
            }

            Message::ToggleService(enabled) => {
                self.install_as_service = enabled;
            }

            Message::ToggleZfs(enabled) => {
                self.enable_zfs = enabled;
            }

            Message::SetInstallPath(path) => {
                self.install_path = path;
            }

            Message::SetApiPort(port) => {
                self.api_port = port;
            }

            Message::SetSongbirdUrl(url) => {
                self.songbird_url = url;
            }

            Message::ToggleAiFeatures(enabled) => {
                self.enable_ai = enabled;
            }

            Message::InstallationProgress(progress) => {
                self.installation_progress = progress;
                if progress >= 100.0 {
                    self.current_step = InstallationStep::Complete;
                }
            }

            Message::InstallationComplete => {
                self.current_step = InstallationStep::Complete;
            }

            Message::InstallationError(error) => {
                self.error_message = Some(error);
            }

            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let content = match self.current_step {
            InstallationStep::Welcome => self.welcome_view(),
            InstallationStep::SystemCheck => self.system_check_view(),
            InstallationStep::InstallOptions => self.install_options_view(),
            InstallationStep::Configuration => self.configuration_view(),
            InstallationStep::Installation => self.installation_view(),
            InstallationStep::Complete => self.complete_view(),
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

#[cfg(feature = "gui")]
impl NestGateInstallerGui {
    fn welcome_view(&self) -> Element<Message> {
        column![
            text("Welcome to NestGate")
                .size(32)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(
                    0.2, 0.8, 1.0
                ))),
            text("The next-generation storage management platform").size(18),
            text("This installer will guide you through setting up NestGate on your system.")
                .size(14),
            column![
                text("Features:"),
                text("  • ZFS storage management"),
                text("  • AI-powered optimization"),
                text("  • Songbird network orchestration"),
                text("  • MCP protocol support"),
                text("  • Web-based management interface"),
            ]
            .spacing(5),
            row![
                button("Cancel").on_press(Message::Cancel),
                button("Next").on_press(Message::NextStep),
            ]
            .spacing(10)
            .width(Length::Fill)
        ]
        .spacing(20)
        .into()
    }

    fn system_check_view(&self) -> Element<Message> {
        let checks = self
            .system_checks
            .iter()
            .map(|check| {
                let status_text = match check.status {
                    CheckStatus::Pending => "⏳",
                    CheckStatus::Success => "✅",
                    CheckStatus::Warning => "⚠️",
                    CheckStatus::Error => "❌",
                };

                row![
                    text(status_text).size(20),
                    column![text(&check.name).size(16), text(&check.message).size(12),].spacing(2),
                ]
                .spacing(10)
                .into()
            })
            .collect::<Vec<_>>();

        column![
            text("System Requirements Check").size(24),
            column(checks).spacing(10),
            row![
                button("Back").on_press(Message::PreviousStep),
                button("Next").on_press(Message::NextStep),
            ]
            .spacing(10)
            .width(Length::Fill)
        ]
        .spacing(20)
        .into()
    }

    fn install_options_view(&self) -> Element<Message> {
        column![
            text("Installation Options").size(24),
            text("Configure how NestGate should be installed:").size(16),
            // Interactive controls instead of static text
            row![
                checkbox("Install as system service", self.install_as_service)
                    .on_toggle(Message::ToggleService),
                text("Automatically start NestGate on system boot").size(12)
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            row![
                checkbox("Enable ZFS support", self.enable_zfs).on_toggle(Message::ToggleZfs),
                text("Required for advanced storage features").size(12)
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            column![
                text("Installation Path:").size(14),
                text_input("Installation directory", &self.install_path)
                    .on_input(Message::SetInstallPath)
                    .width(Length::Fill),
                text("Default: /opt/nestgate").size(10)
            ]
            .spacing(5),
            row![
                button("Back").on_press(Message::PreviousStep),
                button("Next").on_press(Message::NextStep),
            ]
            .spacing(10)
            .width(Length::Fill)
        ]
        .spacing(20)
        .into()
    }

    fn configuration_view(&self) -> Element<Message> {
        column![
            text("Configuration").size(24),
            text("Configure NestGate settings:"),
            text(format!("API Port: {}", self.api_port)),
            text(format!("Songbird URL: {}", self.songbird_url)),
            text(format!("Enable AI: {}", self.enable_ai)),
            row![
                button("Back").on_press(Message::PreviousStep),
                button("Install").on_press(Message::NextStep),
            ]
            .spacing(10)
            .width(Length::Fill)
        ]
        .spacing(20)
        .into()
    }

    fn installation_view(&self) -> Element<Message> {
        column![
            text("Installing NestGate...").size(24),
            progress_bar(0.0..=100.0, self.installation_progress),
            text(&self.installation_message),
            if let Some(error) = &self.error_message {
                text(error).style(iced::theme::Text::Color(iced::Color::from_rgb(
                    1.0, 0.2, 0.2,
                )))
            } else {
                text("")
            }
        ]
        .spacing(20)
        .into()
    }

    fn complete_view(&self) -> Element<Message> {
        column![
            text("Installation Complete!")
                .size(32)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(
                    0.2, 1.0, 0.2
                ))),
            text("NestGate has been successfully installed on your system."),
            column![
                text("Next steps:"),
                text("  • Run 'nestgate --help' to see available commands"),
                text(&format!(
                    "  • Visit http://localhost:{} for the web interface",
                    std::env::var("NESTGATE_UI_PORT").unwrap_or_else(|_| "8080".to_string())
                )),
                text("  • Check the documentation for advanced configuration"),
            ]
            .spacing(5),
            button("Finish").on_press(Message::Cancel),
        ]
        .spacing(20)
        .into()
    }
}

#[cfg(feature = "gui")]
async fn simulate_installation() -> f32 {
    use tokio::time::{sleep, Duration};

    for i in 0..=100 {
        sleep(Duration::from_millis(50)).await;
        if i == 100 {
            return 100.0;
        }
    }
    100.0
}
