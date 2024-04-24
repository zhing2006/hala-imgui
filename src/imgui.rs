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

  font_sampler: std::mem::ManuallyDrop<hala_gfx::HalaSampler>,
  font_descriptor_pool: std::mem::ManuallyDrop<Rc<RefCell<hala_gfx::HalaDescriptorPool>>>,
  font_descriptor_set: std::mem::ManuallyDrop<hala_gfx::HalaDescriptorSet>,

  font_image: Option<hala_gfx::HalaImage>,

  imgui: *mut ImGuiContext,
}

/// The implementation of the drop trait for the ImGUI context.
impl Drop for HalaImGui {

  /// Drop the ImGUI context.
  fn drop(&mut self) {
    self.font_image = None;

    unsafe {
      ImGui_DestroyContext(self.imgui);
      self.imgui = null_mut();

      std::mem::ManuallyDrop::drop(&mut self.font_descriptor_set);
      std::mem::ManuallyDrop::drop(&mut self.font_descriptor_pool);
      std::mem::ManuallyDrop::drop(&mut self.font_sampler);
    }
    log::debug!("ImGUI context dropped.");
  }

}

/// The implementation of the ImGUI context.
impl HalaImGui {

  /// Create a new ImGUI context.
  pub fn new(vk_ctx: Rc<RefCell<hala_gfx::HalaContext>>) -> Result<Self> {
    let (
      font_descriptor_pool,
      font_descriptor_set,
      font_sampler,
    ) = {
      let context = vk_ctx.borrow();

      let font_sampler = hala_gfx::HalaSampler::new(
        Rc::clone(&context.logical_device),
        (hala_gfx::HalaFilter::LINEAR, hala_gfx::HalaFilter::LINEAR),
        hala_gfx::HalaSamplerMipmapMode::LINEAR,
        (hala_gfx::HalaSamplerAddressMode::REPEAT, hala_gfx::HalaSamplerAddressMode::REPEAT, hala_gfx::HalaSamplerAddressMode::REPEAT),
        0.0,
        false,
        1.0,
        (-1000.0, 1000.0),
        "imgui_font.sampler",
      )?;

      let font_descriptor_pool = Rc::new(
        RefCell::new(
          hala_gfx::HalaDescriptorPool::new(
            Rc::clone(&context.logical_device),
            &[
              (hala_gfx::HalaDescriptorType::COMBINED_IMAGE_SAMPLER, 1),
            ],
            1,
            "imgui_font.descpool",
          )?
        )
      );

      let font_descriptor_set = hala_gfx::HalaDescriptorSet::new(
        Rc::clone(&context.logical_device),
        Rc::clone(&font_descriptor_pool),
        hala_gfx::HalaDescriptorSetLayout::new(
          Rc::clone(&context.logical_device),
          &[
            (
              0,
              hala_gfx::HalaDescriptorType::COMBINED_IMAGE_SAMPLER,
              1,
              hala_gfx::HalaShaderStageFlags::FRAGMENT,
              hala_gfx::HalaDescriptorBindingFlags::PARTIALLY_BOUND,
            ),
          ],
          "imgui_font.descsetlayout",
        )?,
        1,
        0,
        "imgui_font.descset",
      )?;

      (
        std::mem::ManuallyDrop::new(font_descriptor_pool),
        std::mem::ManuallyDrop::new(font_descriptor_set),
        std::mem::ManuallyDrop::new(font_sampler),
      )
    };

    // Initialize ImGUI.
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
      font_descriptor_pool,
      font_descriptor_set,
      font_sampler,
      font_image: None,
      imgui,
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

      if self.font_image.is_none() {
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
      let draw_data = ImGui_GetDrawData();
      let is_minimized = (*draw_data).DisplaySize.x <= 0.0 || (*draw_data).DisplaySize.y <= 0.0;
      if !is_minimized {
        // TODO: Draw UI.
      }
    }

    Ok(())
  }

  /// Create the fonts texture.
  fn create_fonts_texture(&mut self) -> Result<()> {
    let context = self.vk_ctx.borrow();

    let (
      upload_size,
      width,
      height,
      pixels,
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
        (width * height * bytes_per_pixel) as u64,
        width as u32,
        height as u32,
        pixels,
      )
    };

    // Create image.
    let font_image = hala_gfx::HalaImage::new_2d(
      Rc::clone(&context.logical_device),
      hala_gfx::HalaImageUsageFlags::SAMPLED | hala_gfx::HalaImageUsageFlags::TRANSFER_DST,
      hala_gfx::HalaFormat::R8G8B8A8_UNORM,
      width,
      height,
      1,
      1,
      hala_gfx::HalaMemoryLocation::GpuOnly,
      "imgui_font.image",
    )?;
    let upload_buffer = hala_gfx::HalaBuffer::new(
      Rc::clone(&context.logical_device),
      upload_size,
      hala_gfx::HalaBufferUsageFlags::TRANSFER_SRC,
      hala_gfx::HalaMemoryLocation::CpuToGpu,
      "imgui_font_upload.buffer",
    )?;

    let font_command_buffers = hala_gfx::HalaCommandBufferSet::new(
      Rc::clone(&context.logical_device),
      Rc::clone(&context.pools),
      hala_gfx::HalaCommandBufferType::GRAPHICS,
      hala_gfx::HalaCommandBufferLevel::PRIMARY,
      1,
      "imgui_font.cmdbuf",
    )?;
    font_image.update_gpu_memory_with_buffer_raw(
      pixels,
      upload_size as usize,
      hala_gfx::HalaPipelineStageFlags2::FRAGMENT_SHADER
        | hala_gfx::HalaPipelineStageFlags2::TRANSFER,
      &upload_buffer,
      &font_command_buffers,
    )?;

    // Update descriptor set.
    self.font_descriptor_set.update_combined_image_samplers(
      0,
      0,
      &[
        (&font_image, &(*self.font_sampler)),
      ],
    );

    self.font_image = Some(font_image);

    // Store identifier.
    unsafe {
      let io = ImGui_GetIO();

      (*(*io).Fonts).TexID = self.font_descriptor_set.handle(0) as ImTextureID;
    }

    Ok(())
  }

}