use slt::{AlertLevel, Border, ButtonVariant, Color, Context, ListState, RunConfig, RichLogState, SpinnerState, Theme};
use std::{
    io::{Read, BufRead, BufReader, Write},
    process::{Child, Command, Stdio},
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

fn forward_lines<R>(
    reader: R,
    tx: Sender<String>,
    prefix: &'static str,
)
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let reader = BufReader::new(reader);

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if tx.send(format!("[{prefix}] {line}")).is_err() {
                        break;
                    }
                }
                Err(error) => {
                    let _ = tx.send(format!(
                        "[ERROR] Log stream: {error}"
                    ));
                    break;
                }
            }
        }
    });
}

fn main() -> std::io::Result<()> {
    let mut nav = ListState::new(vec!["Overview", "General settings"]);
    let mut spinner = SpinnerState::dots();
    let mut richlog = RichLogState::new();
    richlog.max_entries = Some(200);

    let mut server_running = false;
    let mut connected = false;
    let mut elapsed = Duration::ZERO;
    let mut started = Instant::now();

    let mut child_process: Option<Child> = None;
    let (log_tx, log_rx) = mpsc::channel::<String>();

    slt::run_with(
        RunConfig::default()
            .theme(Theme::dark())
            .mouse(true)
            .tick_rate(Duration::from_millis(16)),
        |ui: &mut Context| {
            if ui.key('q') { ui.quit(); }

            for line in log_rx.try_iter() {
                richlog.push_plain(&line);
            }

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
                    if child_process.is_some() {
                        if ui
                            .button_with("Stop server", ButtonVariant::Outline)
                            .clicked
                        {
                            server_running = false;
                            if let Some(mut child) = child_process.take() {
                                richlog.push_plain("[INFO] Stopping server...");

                                match child.kill() {
                                    Ok(()) => {
                                        let _ = child.wait();
                                        richlog.push_plain("[INFO] Server stopped.");
                                    }
                                    Err(error) => {
                                        richlog.push_plain(&format!(
                                            "[ERROR] Could not stop server: {error}"
                                        ));
                                    }
                                }
                            }
                        }
                    } else if ui
                        .button_with("Start server", ButtonVariant::Outline)
                        .clicked
                    {
                        elapsed = Duration::ZERO;
                        richlog.push_plain("[INFO] Starting server...");
                        server_running = true;

                        match Command::new("uxplay")
                            .args(["-p"])
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .spawn()
                        {
                            Ok(mut child) => {
                                if let Some(stdout) = child.stdout.take() {
                                    forward_lines(stdout, log_tx.clone(), "OUT");
                                }

                                if let Some(stderr) = child.stderr.take() {
                                    forward_lines(stderr, log_tx.clone(), "ERR");
                                }

                                richlog.push_plain(&format!(
                                    "[INFO] Process started (PID {}).",
                                    child.id()
                                ));

                                child_process = Some(child);
                            }
                            Err(error) => {
                                richlog.push_plain(&format!(
                                    "[ERROR] Failed to start server: {error}"
                                ));
                            }
                        }
                    }
                        ui.bordered(Border::Single).title("Server Logs").p(1).grow(1).col(|ui| {
                            ui.rich_log(&mut richlog);
                        });
                    });
                });
                let _ = ui.help(&[("q", "quit"), ("Tab", "focus"), ("Enter", "select"), ("r", "refresh"), ("?", "help")]);
            });
        },
    )
}
