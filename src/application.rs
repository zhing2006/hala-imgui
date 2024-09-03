use std::path::Path;

use anyhow::Result;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::encode::pattern::PatternEncoder;

use winit::{
  application::ApplicationHandler,
  event::{WindowEvent, Ime},
  event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
  window::{WindowButtons, Window, WindowId},
};

use crate::HalaImGui;

/// The application context trait.
pub trait HalaApplicationContextTrait {

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

  /// Get the ImGui context ref.
  /// return: The ImGui context reference.
  fn get_imgui(&self) -> Option<&HalaImGui>;

  /// Get the ImGui context mut.
  /// return: The ImGui context mutable reference.
  fn get_imgui_mut(&mut self) -> Option<&mut HalaImGui>;

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

  /// Handle the keyboard event.
  /// param key: The key.
  /// param is_pressed: The key is pressed or not.
  /// return: The result.
  fn on_keyboard_event(&mut self, _key: winit::keyboard::KeyCode, _is_pressed: bool) -> Result<()> {
    Ok(())
  }
  /// Handle the mouse button event.
  /// param button: The button.
  /// param is_pressed: The button is pressed or not.
  /// return: The result.
  fn on_mouse_button_event(&mut self, _button: winit::event::MouseButton, _is_pressed: bool) -> Result<()> {
    Ok(())
  }
  /// Handle the mouse cursor move event.
  /// param x: The x position.
  /// param y: The y position.
  /// return: The result.
  fn on_mouse_cursor_event(&mut self, _x: f32, _y: f32) -> Result<()> {
    Ok(())
  }
  /// Handle the mouse wheel event.
  /// param h: The horizontal value.
  /// param v: The vertical value.
  /// return: The result.
  fn on_mouse_wheel_event(&mut self, _h: f32, _v: f32) -> Result<()> {
    Ok(())
  }

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

}

/// The application struct.
pub struct HalaApplication {
  pub context: Box<dyn HalaApplicationContextTrait>,
  window: Option<Window>,
  last_time: std::time::Instant,
}

/// Implement the ApplicationHandler trait for the HalaApplication struct.
impl ApplicationHandler for HalaApplication {

  /// Emitted when the event loop is about to block and wait for new events.
  fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
    let window = self.window.as_ref().unwrap();
    window.request_redraw();
  }

  /// Emitted when the application has been resumed.
  /// param event_loop: The event loop that the application is running on.
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let win_size = self.context.get_window_size();
    let win_attr = Window::default_attributes()
      .with_title(self.context.get_window_title())
      .with_inner_size(win_size)
      .with_resizable(false)
      .with_enabled_buttons(WindowButtons::CLOSE);
    self.window = Some(event_loop.create_window(win_attr).unwrap());

    log::debug!("Create window \"{}\" with size {}x{}.", self.context.get_window_title(), win_size.width, win_size.height);

    // NOTICE: We only run on the platform that not support suspend/resume.
    // So here is safe to call before_run.
    self.context.before_run(win_size.width, win_size.height, self.window.as_ref().unwrap()).unwrap();

    self.last_time = std::time::Instant::now();
  }

  /// Emitted when the OS sends an event to a winit window.
  /// param event_loop: The event loop that the application is running on.
  /// param id: The ID of the window that the event was sent to.
  /// param event: The event that was sent to the window.
  fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
    let window = self.window.as_ref().unwrap();

    match event {
      WindowEvent::CloseRequested if window_id == window.id() => {
        self.context.after_run();
        event_loop.exit()
      },
      WindowEvent::RedrawRequested if window_id == window.id() => {
        let now = std::time::Instant::now();
        let duration = now - self.last_time;
        let delta_time = duration.as_secs_f64();
        self.last_time = std::time::Instant::now();
        let window_size = window.inner_size();
        match self.context.update(delta_time, window_size.width, window_size.height) {
          Ok(_) => {
            match self.context.render() {
              Ok(_) => (),
              Err(e) => {
                log::error!("Failed to render the application: {}", e);
                event_loop.exit()
              },
            }
          },
          Err(e) => {
            log::error!("Failed to update the application: {}", e);
            event_loop.exit()
          },
        }
      },
      WindowEvent::ModifiersChanged(mods) if window_id == window.id() => {
        let imgui = self.context.get_imgui_mut();
        if let Some(imgui) = imgui {
          imgui.add_key_event(imgui::Key::ModCtrl, mods.state().control_key());
          imgui.add_key_event(imgui::Key::ModShift, mods.state().shift_key());
          imgui.add_key_event(imgui::Key::ModAlt, mods.state().alt_key());
          imgui.add_key_event(imgui::Key::ModSuper, mods.state().super_key());
        }
      },
      WindowEvent::KeyboardInput {
        event: winit::event::KeyEvent {
          physical_key,
          text,
          state,
          ..
        },
        is_synthetic: false,
        ..
      } => {
        let mut is_ui_processed = false;
        let imgui = self.context.get_imgui_mut();
        if let Some(imgui) = imgui {
          let is_pressed = state == winit::event::ElementState::Pressed;
          if let Some(key) = HalaImGui::to_key(physical_key) {
            imgui.add_key_event(key, is_pressed);

            if let winit::keyboard::PhysicalKey::Code(keycode) = physical_key {
              let kmod = match keycode {
                winit::keyboard::KeyCode::ControlLeft | winit::keyboard::KeyCode::ControlRight => Some(imgui::Key::ModCtrl),
                winit::keyboard::KeyCode::ShiftLeft | winit::keyboard::KeyCode::ShiftRight => Some(imgui::Key::ModShift),
                winit::keyboard::KeyCode::AltLeft | winit::keyboard::KeyCode::AltRight => Some(imgui::Key::ModAlt),
                winit::keyboard::KeyCode::SuperLeft | winit::keyboard::KeyCode::SuperRight => Some(imgui::Key::ModSuper),
                _ => None,
              };
              if let Some(kmod) = kmod {
                imgui.add_key_event(kmod, is_pressed);
              }
            }
          }
          if is_pressed {
            if let Some(text) = text {
              for c in text.chars() {
                imgui.add_input_character(c as u32);
              }
            }
          }
          is_ui_processed = imgui.want_capture_keyboard();
        }
        if !is_ui_processed {
          if let winit::keyboard::PhysicalKey::Code(keycode) = physical_key {
            match self.context.on_keyboard_event(keycode, state == winit::event::ElementState::Pressed) {
              Ok(_) => (),
              Err(e) => {
                log::error!("Failed to handle keyboard event: {}", e);
                event_loop.exit()
              },
            }
          }
        }
      },
      WindowEvent::Ime(Ime::Commit(text)) => {
        let imgui = self.context.get_imgui_mut();
        if let Some(imgui) = imgui {
          for c in text.chars() {
            imgui.add_input_character(c as u32);
          }
        }
      },
      WindowEvent::CursorMoved {
        position,
        ..
      } => {
        let mut is_ui_processed = false;
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let imgui = self.context.get_imgui_mut();
        if let Some(imgui) = imgui {
          let scale = window.scale_factor();
          let position = position.to_logical::<f32>(scale);
          x = position.x;
          y = position.y;
          imgui.add_mouse_pos_event(x, y);
          is_ui_processed = imgui.want_capture_mouse();
        }
        if !is_ui_processed {
          match self.context.on_mouse_cursor_event(x, y) {
            Ok(_) => (),
            Err(e) => {
              log::error!("Failed to handle mouse move event: {}", e);
              event_loop.exit()
            },
          }
        }
      },
      WindowEvent::CursorLeft { .. } => {
        let mut is_ui_processed = false;
        let imgui = self.context.get_imgui_mut();
        if let Some(imgui) = imgui {
          imgui.add_mouse_pos_event(f32::MAX, f32::MAX);
          is_ui_processed = imgui.want_capture_mouse();
        }
        if !is_ui_processed {
          match self.context.on_mouse_cursor_event(f32::MAX, f32::MAX) {
            Ok(_) => (),
            Err(e) => {
              log::error!("Failed to handle mouse move event: {}", e);
              event_loop.exit()
            },
          }
        }
      },
      WindowEvent::MouseInput {
        state,
        button,
        ..
      } => {
        let mut is_ui_processed = false;
        let imgui = self.context.get_imgui_mut();
        if let Some(imgui) = imgui {
          if let Some(button) = HalaImGui::to_button(button) {
            let is_pressed = state == winit::event::ElementState::Pressed;
            imgui.add_mouse_button_event(button, is_pressed);
            is_ui_processed = imgui.want_capture_mouse();
          }
        }
        if !is_ui_processed {
          match self.context.on_mouse_button_event(button, state == winit::event::ElementState::Pressed) {
            Ok(_) => (),
            Err(e) => {
              log::error!("Failed to handle mouse button event: {}", e);
              event_loop.exit()
            },
          }
        }
      },
      WindowEvent::MouseWheel {
        delta,
        phase: winit::event::TouchPhase::Moved,
        ..
      } => {
        let mut is_ui_processed = false;
        let mut h: f32 = 0.0;
        let mut v: f32 = 0.0;
        let imgui = self.context.get_imgui_mut();
        if let Some(imgui) = imgui {
          (h, v) = match delta {
            winit::event::MouseScrollDelta::LineDelta(h, v) => (h, v),
            winit::event::MouseScrollDelta::PixelDelta(pos) => {
              let scale = imgui.get_display_framebuffer_scale();
              let f_scale = imgui.get_font_size();
              let scale = scale[0] * f_scale;
              (pos.x as f32 / scale, pos.y as f32 / scale)
            },
          };
          imgui.add_mouse_wheel_event(h, v);
          is_ui_processed = imgui.want_capture_mouse();
        }
        if !is_ui_processed {
          match self.context.on_mouse_wheel_event(h, v) {
            Ok(_) => (),
            Err(e) => {
              log::error!("Failed to handle mouse wheel event: {}", e);
              event_loop.exit()
            },
          }
        }
      },
      WindowEvent::Focused(is_focused) => {
        let imgui = self.context.get_imgui_mut();
        if let Some(imgui) = imgui {
          imgui.add_focus_event(is_focused);
        }
      },
      _ => (),
    }
  }

}

/// Implement the HalaApplication struct.
impl HalaApplication {

  /// Create a new HalaApplication instance.
  pub fn new(context: Box<dyn HalaApplicationContextTrait>) -> Self {
    Self {
      context,
      window: None,
      last_time: std::time::Instant::now(),
    }
  }

  /// Run the application.
  pub fn run(&mut self) -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run_app(self)?;

    Ok(())
  }

}