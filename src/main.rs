use slt::{AlertLevel, Border, ButtonVariant, Color, Context, ListState, RunConfig, ScrollState, SpinnerState, Theme};
use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    let mut nav = ListState::new(vec!["Overview", "General settings"]);
    let mut spinner = SpinnerState::dots();
    let mut scrollable = ScrollState::new();
    let mut server_running = false;
    let mut connected = false;
    let mut elapsed = Duration::ZERO;
    let mut started = Instant::now();

    slt::run_with(
        RunConfig::default()
            .theme(Theme::dark())
            .mouse(true)
            .tick_rate(Duration::from_millis(16)),
        |ui: &mut Context| {
            if ui.key('q') { ui.quit(); }

            ui.container().gap(1).grow(1).col(|ui| {
                ui.bordered(Border::Rounded).px(2).gap(1).row(|ui| {
                    if server_running {
                        elapsed += started.elapsed();
                        started = Instant::now();
                    }
                    ui.timer_display(elapsed);
                    ui.spacer();
                    ui.text("UxPlay-TUI").bold().fg(Color::Cyan);
                    ui.badge("Live");
                });
                ui.container().gap(1).grow(1).row(|ui| {
                    ui.bordered(Border::Single).w(30).pad(1).gap(1).col(|ui| {
                        ui.divider_text("Navigation");
                        let _ = ui.list(&mut nav);
                        ui.spacer();
                        ui.divider_text("Quick Facts");
                        ui.definition_list(&[("Version", "[version]"), ("MAC", "[xx:xx:xx:xx:xx:xx]"), ("UDP", "[ports]"), ("TCP", "[ports]")]);
                        ui.spacer();
                        ui.divider_text("Client Info");
                        ui.container().col(|ui| {
                            ui.empty_state("No connection yet", "Connect to a client!");
                        });
                        ui.spacer();
                        ui.separator();
                        if !connected {
                            ui.container().col(|ui| {
                                ui.spinner(&spinner);
                                ui.text("Waiting for connections..").wrap();
                            });
                        } else {
                        ui.container().col(|ui| {
                            ui.badge("Connection successfull!");
                        });
                        }
                    });
                    ui.container().gap(1).grow(1).col(|ui| {
                        ui.container().gap(1).row(|ui| {
                            if server_running {
                                let _ = ui.alert("Running", AlertLevel::Success);
                            } else {
                                let _ = ui.alert("Not running", AlertLevel::Warning);
                            }
                        });
                        if server_running {
                            if ui.button_with("Stop server", ButtonVariant::Outline).clicked {
                                server_running = false;
                            }
                        } else {
                            if ui.button_with("Start server", ButtonVariant::Outline).clicked {
                                elapsed = Duration::ZERO;
                                server_running = true;
                                started = Instant::now();
                            }
                        }
                        ui.scrollable(&mut scrollable).h(20).col(|ui| {});
                    });
                });
                let _ = ui.help(&[("q", "quit"), ("Tab", "focus"), ("Enter", "select"), ("r", "refresh"), ("?", "help")]);
            });
        },
    )
}
