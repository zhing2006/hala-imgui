use std::path::Path;

use anyhow::Result;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::encode::pattern::PatternEncoder;

use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{WindowBuilder, WindowButtons},
};

/// The application trait.
pub trait HalaApplication {

  /// Get the file log format string.
  /// return: The format string.
  fn get_log_file_fmt(&self) -> &str;
  /// Get the log file path.
  /// return: The log file path.
  fn get_log_file(&self) -> &Path;
  /// Get the log file size.
  /// return: The log file size.
  fn get_log_file_size(&self) -> u64;
  /// Get the log file roller count.
  /// return: The log file roller count.
  fn get_log_file_roller_count(&self) -> u32;
  /// Get the console log format string.
  /// return: The format string.
  fn get_log_console_fmt(&self) -> &str;

  /// Get the window title.
  /// return: The window title.
  fn get_window_title(&self) -> &str;
  /// Get the window size.
  /// return: The window size.
  fn get_window_size(&self) -> winit::dpi::PhysicalSize<u32>;

  /// The before run function.
  /// param width: The width of the window.
  /// param height: The height of the window.
  /// param window: The window.
  /// return: The result.
  fn before_run(&mut self, width: u32, height: u32, window: &winit::window::Window) -> Result<()>;
  /// The after run function.
  fn after_run(&mut self);
  /// The update function.
  /// param delta_time: The delta time.
  /// param width: The width of the window.
  /// param height: The height of the window.
  /// return: The result.
  fn update(&mut self, delta_time: f64, width: u32, height: u32) -> Result<()>;
  /// The render function.
  /// return: The result.
  fn render(&mut self) -> Result<()>;

  /// Initialize the log system.
  fn init_log(&self) -> Result<()> where Self: Sized {
    let console_pattern_encoder = Box::new(
      PatternEncoder::new(self.get_log_console_fmt())
    );
    let file_pattern_encoder = Box::new(
      PatternEncoder::new(self.get_log_file_fmt())
    );
    let stdout = ConsoleAppender::builder()
      .encoder(console_pattern_encoder)
      .build();
    let rolling_file = RollingFileAppender::builder()
      .encoder(file_pattern_encoder)
      .append(true)
      .build(
        self.get_log_file(),
        Box::new(log4rs::append::rolling_file::policy::compound::CompoundPolicy::new(
          Box::new(log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger::new(self.get_log_file_size())),
          Box::new(log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller::builder()
            .build(&format!("{}.{}.gz", self.get_log_file().to_string_lossy(), "{}"), self.get_log_file_roller_count())?
          )
        ),
      ))?;
    let config_builder = log4rs::config::Config::builder()
      .appender(log4rs::config::Appender::builder().build("stdout", Box::new(stdout)))
      .appender(log4rs::config::Appender::builder().build("rolling_file", Box::new(rolling_file)));
    let config = if cfg!(debug_assertions) {
      config_builder.build(log4rs::config::Root::builder().appenders(["stdout", "rolling_file"]).build(LevelFilter::Debug))
    } else {
      config_builder.build(log4rs::config::Root::builder().appenders(["stdout", "rolling_file"]).build(LevelFilter::Info))
    }?;
    let _ = log4rs::init_config(config)?;

    log::info!("Log system initialized.");
    Ok(())
  }

  /// Initialize the application.
  /// return: The result of the initialization.
  fn init(&self) -> Result<()> where Self: Sized {
    self.init_log()?;

    Ok(())
  }

  /// Run the application.
  /// return: The result of the running.
  fn run(&mut self) -> Result<()> where Self: Sized {
    let event_loop = EventLoop::new()?;
    let win_size = self.get_window_size();
    let window = WindowBuilder::new()
      .with_title(self.get_window_title())
      .with_inner_size(win_size)
      .with_resizable(false)
      .with_enabled_buttons(WindowButtons::CLOSE)
      .build(&event_loop)?;
    log::debug!("Create window \"{}\" with size {}x{}.", self.get_window_title(), win_size.width, win_size.height);

    self.before_run(win_size.width, win_size.height, &window)?;

    let mut last_time = std::time::Instant::now();

    event_loop.run(move |event, elwt| {
      elwt.set_control_flow(ControlFlow::Poll);
      match event {
        Event::WindowEvent {
          event,
          window_id,
        } => match event {
          WindowEvent::CloseRequested if window_id == window.id() => {
            self.after_run();
            elwt.exit()
          },
          WindowEvent::RedrawRequested if window_id == window.id() => {
            let now = std::time::Instant::now();
            let duration = now - last_time;
            let delta_time = duration.as_secs_f64();
            last_time = std::time::Instant::now();
            let window_size = window.inner_size();
            match self.update(delta_time, window_size.width, window_size.height) {
              Ok(_) => {
                match self.render() {
                  Ok(_) => (),
                  Err(e) => {
                    log::error!("Failed to render the application: {}", e);
                    elwt.exit()
                  },
                }
              },
              Err(e) => {
                log::error!("Failed to update the application: {}", e);
                elwt.exit()
              },
            }
          },
          _ => (),
        },
        Event::AboutToWait => window.request_redraw(),
        _ => (),
      }
    })?;

    Ok(())
  }

}