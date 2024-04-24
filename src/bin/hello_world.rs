use std::{
  ptr::null_mut,
  rc::Rc,
  cell::RefCell,
};

use anyhow::Result;

use easy_imgui_sys::*;

use hala_imgui::{
  HalaApplication,
  HalaImGui,
};

/// The hello world renderer.
struct HelloWorldRenderer {
  context: std::mem::ManuallyDrop<Rc<RefCell<hala_gfx::HalaContext>>>,
  graphics_command_buffers: std::mem::ManuallyDrop<hala_gfx::HalaCommandBufferSet>,

  image_index: usize,
}

/// The implementation of the drop trait for the hello world renderer.
impl Drop for HelloWorldRenderer {

  /// Drop the hello world renderer.
  fn drop(&mut self) {
    unsafe {
      std::mem::ManuallyDrop::drop(&mut self.graphics_command_buffers);
      std::mem::ManuallyDrop::drop(&mut self.context);
    }
    log::debug!("Hello World renderer dropped.");
  }

}

/// The implementation of the hello world renderer.
impl HelloWorldRenderer {

  /// Create a new hello world renderer.
  pub fn new(name: &str, gpu_req: &hala_gfx::HalaGPURequirements, window: &winit::window::Window) -> Result<Self> {
    log::debug!("Create a new Renderer \"{}\".", name);
    let context = hala_gfx::HalaContext::new(name, gpu_req, window)?;

    let graphics_command_buffers = hala_gfx::HalaCommandBufferSet::new(
      Rc::clone(&context.logical_device),
      Rc::clone(&context.pools),
      hala_gfx::HalaCommandBufferType::GRAPHICS,
      hala_gfx::HalaCommandBufferLevel::PRIMARY,
      context.swapchain.num_of_images,
      "main_graphics.cmdbuf",
    )?;

    log::debug!("Hello World renderer created.");
    Ok(
      Self {
        context: std::mem::ManuallyDrop::new(Rc::new(RefCell::new(context))),
        graphics_command_buffers: std::mem::ManuallyDrop::new(graphics_command_buffers),

        image_index: 0,
      }
    )
  }

  /// Wait the renderer idle.
  /// return: The result.
  pub fn wait_idle(&self) -> Result<()> {
    self.context.borrow().logical_device.borrow().wait_idle()?;

    Ok(())
  }

  /// Update the renderer.
  /// param delta_time: The delta time.
  /// return: The result.
  pub fn update<F>(&mut self, _delta_time: f64, ui_fn: F) -> Result<()>
    where F: FnOnce(usize, &hala_gfx::HalaCommandBufferSet) -> Result<(), hala_gfx::HalaGfxError>
  {
    self.image_index = self.context.borrow().prepare_frame()?;
    self.context.borrow().record_graphics_command_buffer(
      self.image_index,
      &self.graphics_command_buffers,
      Some(([25.0 / 255.0, 118.0 / 255.0, 210.0 / 255.0, 1.0], 1.0, 0)),
      |index, command_buffers| {
        ui_fn(index, command_buffers)?;

        Ok(())
      },
      None,
      |_, _| Ok(false),
    )?;
    Ok(())
  }

  /// Rendering.
  /// return: The result.
  pub fn render(&mut self) -> Result<()> {
    self.context.borrow_mut().submit_and_present_frame(
      self.image_index, &self.graphics_command_buffers
    )?;

    Ok(())
  }

}

/// The hello world application.
struct HelloWorldApp {
  renderer: Option<HelloWorldRenderer>,
  imgui: Option<hala_imgui::HalaImGui>,
}

/// The implementation of the application trait for the hello world application.
impl HalaApplication for HelloWorldApp {

  fn get_log_console_fmt(&self) -> &str {
    "{d(%H:%M:%S)} {h({l:<5})} {t:<20.20} - {m}{n}"
  }
  fn get_log_file_fmt(&self) -> &str {
    "{d(%Y-%m-%d %H:%M:%S)} {h({l:<5})} {f}:{L} - {m}{n}"
  }
  fn get_log_file(&self) -> &std::path::Path {
    std::path::Path::new("./logs/hello_world.log")
  }
  fn get_log_file_size(&self) -> u64 {
    1024 * 1024 /* 1MB */
  }
  fn get_log_file_roller_count(&self) -> u32 {
    5
  }

  fn get_window_title(&self) -> &str {
    "Hello World"
  }
  fn get_window_size(&self) -> winit::dpi::PhysicalSize<u32> {
    winit::dpi::PhysicalSize::new(800, 600)
  }

  fn before_run(&mut self, width: u32, height: u32, window: &winit::window::Window) -> Result<()> {
    let gpu_req = hala_gfx::HalaGPURequirements {
      width,
      height,
      version: (1, 3, 0),
      require_ray_tracing: false,
      require_10bits_output: false,
      is_low_latency: true,
      require_depth: true,
      ..Default::default()
    };
    let renderer = HelloWorldRenderer::new(
      self.get_window_title(),
      &gpu_req,
      window
    )?;
    self.imgui = Some(HalaImGui::new(
      Rc::clone(&(*renderer.context))
    )?);
    self.renderer = Some(renderer);

    Ok(())
  }

  fn after_run(&mut self) {
    self.imgui = None;
    if let Some(renderer) = self.renderer.take() {
      renderer.wait_idle().expect("Failed to wait the renderer idle.");
    }
  }

  fn update(&mut self, delta_time: f64, width: u32, height: u32) -> Result<()> {
    if let Some(imgui) = self.imgui.as_mut() {
      imgui.begin_frame(delta_time, width, height)?;
    }
    self.ui();
    if let Some(imgui) = self.imgui.as_mut() {
      imgui.end_frame()?;
    }

    if let Some(renderer) = self.renderer.as_mut() {
      renderer.update(delta_time, |index, command_buffers| {
        if let Some(imgui) = self.imgui.as_mut() {
          imgui.draw(index, command_buffers)?;
        }

        Ok(())
      })?;
    }

    Ok(())
  }

  fn render(&mut self) -> Result<()> {
    if let Some(renderer) = self.renderer.as_mut() {
      renderer.render()?;
    }

    Ok(())
  }

}

/// The implementation of the hello world application.
impl HelloWorldApp {

  /// Create a new hello world application.
  pub fn new() -> Self {
    Self {
      renderer: None,
      imgui: None,
    }
  }

  fn ui(&mut self) {
    unsafe {
      ImGui_Begin(
        "Hello, World!".as_ptr() as *const i8,
        null_mut(),
        ImGuiWindowFlags_::ImGuiWindowFlags_None.0
      );

      ImGui_Text("Hello, World!".as_ptr() as *const i8);

      ImGui_End();
    }
  }

}

/// the normal main function.
fn main() -> Result<()> {
  let mut app = HelloWorldApp::new();
  app.init()?;
  app.run()?;

  Ok(())
}