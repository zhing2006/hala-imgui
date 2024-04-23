use std::{
  cell::RefCell,
  ptr::null_mut,
  rc::Rc
};

use anyhow::{Ok, Result};

use easy_imgui_sys::*;

/// The ImGUI context.
pub struct HalaImGui {
  pub(crate) vk_ctx: Rc<RefCell<hala_gfx::HalaContext>>,

  imgui: *mut ImGuiContext,

  font_command_buffers: Option<hala_gfx::HalaCommandBufferSet>,
  font_image: Option<hala_gfx::HalaImage>,
  font_descriptor_set: Option<hala_gfx::HalaDescriptorSet>,
}

/// The implementation of the drop trait for the ImGUI context.
impl Drop for HalaImGui {

  /// Drop the ImGUI context.
  fn drop(&mut self) {
    self.font_descriptor_set = None;
    self.font_image = None;
    self.font_command_buffers = None;

    unsafe {
      ImGui_DestroyContext(self.imgui);
      self.imgui = null_mut();
    }
    log::debug!("ImGUI context dropped.");
  }

}

/// The implementation of the ImGUI context.
impl HalaImGui {

  /// Create a new ImGUI context.
  pub fn new(vk_ctx: Rc<RefCell<hala_gfx::HalaContext>>) -> Result<Self> {
    let imgui = unsafe {
      let imgui = ImGui_CreateContext(null_mut());
      ImGui_StyleColorsDark(null_mut());
      imgui
    };

    unsafe {
      let io = ImGui_GetIO();
      let conf_flags = ImGuiConfigFlags_::ImGuiConfigFlags_NavEnableKeyboard
        | ImGuiConfigFlags_::ImGuiConfigFlags_NavEnableGamepad;
      (*io).ConfigFlags = conf_flags.0;
    };

    log::debug!("ImGUI context created.");
    Ok(Self {
      vk_ctx,
      imgui,
      font_command_buffers: None,
      font_image: None,
      font_descriptor_set: None,
    })
  }

  /// Begin the ImGUI frame.
  /// param delta_time: The delta time.
  /// param width: The width of the window.
  /// param height: The height of the window.
  pub fn new_frame(&mut self, delta_time: f64, width: u32, height: u32) -> Result<()> {
    unsafe {
      let io = ImGui_GetIO();
      (*io).DeltaTime = delta_time as f32;
      (*io).DisplaySize = ImVec2 { x: width as f32, y: height as f32 };
      (*io).DisplayFramebufferScale = ImVec2 { x: 1.0, y: 1.0 };

      if self.font_descriptor_set.is_none() {
        self.create_fonts_texture()?;
      }

      ImGui_NewFrame();
    }

    Ok(())
  }

  /// Render the ImGUI frame.
  pub fn render(&self) -> Result<()> {
    unsafe {
      ImGui_Render();
    }

    Ok(())
  }

  /// Create the fonts texture.
  fn create_fonts_texture(&mut self) -> Result<()> {
    let context = self.vk_ctx.borrow();

    // Create command buffer.
    if self.font_command_buffers.is_none() {
      self.font_command_buffers = Some(hala_gfx::HalaCommandBufferSet::new(
        Rc::clone(&context.logical_device),
        Rc::clone(&context.pools),
      hala_gfx::HalaCommandBufferType::GRAPHICS,
      hala_gfx::HalaCommandBufferLevel::PRIMARY,
      1,
      "imgui_font.cmdbuf",
      )?);
    }
    let font_command_buffers = self.font_command_buffers.as_ref()
      .ok_or(anyhow::anyhow!("Failed to get the font command buffers."))?;

    let (
      _upload_size,
      width,
      height
     ) = unsafe {
      let io = ImGui_GetIO();
      let mut pixels: *mut u8 = null_mut();
      let mut width: i32 = 0;
      let mut height: i32 = 0;
      let mut bytes_per_pixel: i32 = 0;
      ImFontAtlas_GetTexDataAsRGBA32(
        (*io).Fonts,
        &mut pixels,
        &mut width,
        &mut height,
        &mut bytes_per_pixel);
      (
        (width * height * bytes_per_pixel) as usize,
        width as u32,
        height as u32
      )
    };

    // Create image.
    self.font_image = Some(hala_gfx::HalaImage::new_2d(
      Rc::clone(&context.logical_device),
      hala_gfx::HalaImageUsageFlags::SAMPLED | hala_gfx::HalaImageUsageFlags::TRANSFER_DST,
      hala_gfx::HalaFormat::R8G8B8A8_UNORM,
      width,
      height,
      1,
      1,
      hala_gfx::HalaMemoryLocation::GpuOnly,
      "imgui_font.image",
    )?);

    // Start command buffer.
    font_command_buffers.reset(0, true)?;
    font_command_buffers.begin(0, hala_gfx::HalaCommandBufferUsageFlags::ONE_TIME_SUBMIT)?;

    // End command buffer.
    font_command_buffers.end(0)?;

    context.logical_device.borrow().graphics_submit(font_command_buffers, 0, 0)?;
    context.logical_device.borrow().wait_idle()?;

    self.font_descriptor_set = None;

    Ok(())
  }

  // /// Add a texture.
  // /// param sampler: The sampler.
  // /// param image: The image.
  // /// return: The result.
  // fn add_texture(&mut self, sampler: &hala_gfx::HalaSampler, image: &hala_gfx::HalaImage,) -> Result<()> {
  //   Ok(())
  // }

}