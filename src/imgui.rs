use std::{
  cell::RefCell,
  ptr::{null_mut, null},
  rc::Rc
};

use anyhow::{Ok, Result};

use easy_imgui_sys::*;

// # glslangValidator -V -x -o glsl_shader.vert.u32 glsl_shader.vert
/*
#version 450 core
layout(location = 0) in vec2 aPos;
layout(location = 1) in vec2 aUV;
layout(location = 2) in vec4 aColor;
layout(push_constant) uniform uPushConstant { vec2 uScale; vec2 uTranslate; } pc;

out gl_PerVertex { vec4 gl_Position; };
layout(location = 0) out struct { vec4 Color; vec2 UV; } Out;

void main()
{
    Out.Color = aColor;
    Out.UV = aUV;
    gl_Position = vec4(aPos * pc.uScale + pc.uTranslate, 0, 1);
}
*/
const __GLSL_SHADER_VERT_SPV: [u32; 324] = [
  0x07230203,0x00010000,0x00080001,0x0000002e,0x00000000,0x00020011,0x00000001,0x0006000b,
  0x00000001,0x4c534c47,0x6474732e,0x3035342e,0x00000000,0x0003000e,0x00000000,0x00000001,
  0x000a000f,0x00000000,0x00000004,0x6e69616d,0x00000000,0x0000000b,0x0000000f,0x00000015,
  0x0000001b,0x0000001c,0x00030003,0x00000002,0x000001c2,0x00040005,0x00000004,0x6e69616d,
  0x00000000,0x00030005,0x00000009,0x00000000,0x00050006,0x00000009,0x00000000,0x6f6c6f43,
  0x00000072,0x00040006,0x00000009,0x00000001,0x00005655,0x00030005,0x0000000b,0x0074754f,
  0x00040005,0x0000000f,0x6c6f4361,0x0000726f,0x00030005,0x00000015,0x00565561,0x00060005,
  0x00000019,0x505f6c67,0x65567265,0x78657472,0x00000000,0x00060006,0x00000019,0x00000000,
  0x505f6c67,0x7469736f,0x006e6f69,0x00030005,0x0000001b,0x00000000,0x00040005,0x0000001c,
  0x736f5061,0x00000000,0x00060005,0x0000001e,0x73755075,0x6e6f4368,0x6e617473,0x00000074,
  0x00050006,0x0000001e,0x00000000,0x61635375,0x0000656c,0x00060006,0x0000001e,0x00000001,
  0x61725475,0x616c736e,0x00006574,0x00030005,0x00000020,0x00006370,0x00040047,0x0000000b,
  0x0000001e,0x00000000,0x00040047,0x0000000f,0x0000001e,0x00000002,0x00040047,0x00000015,
  0x0000001e,0x00000001,0x00050048,0x00000019,0x00000000,0x0000000b,0x00000000,0x00030047,
  0x00000019,0x00000002,0x00040047,0x0000001c,0x0000001e,0x00000000,0x00050048,0x0000001e,
  0x00000000,0x00000023,0x00000000,0x00050048,0x0000001e,0x00000001,0x00000023,0x00000008,
  0x00030047,0x0000001e,0x00000002,0x00020013,0x00000002,0x00030021,0x00000003,0x00000002,
  0x00030016,0x00000006,0x00000020,0x00040017,0x00000007,0x00000006,0x00000004,0x00040017,
  0x00000008,0x00000006,0x00000002,0x0004001e,0x00000009,0x00000007,0x00000008,0x00040020,
  0x0000000a,0x00000003,0x00000009,0x0004003b,0x0000000a,0x0000000b,0x00000003,0x00040015,
  0x0000000c,0x00000020,0x00000001,0x0004002b,0x0000000c,0x0000000d,0x00000000,0x00040020,
  0x0000000e,0x00000001,0x00000007,0x0004003b,0x0000000e,0x0000000f,0x00000001,0x00040020,
  0x00000011,0x00000003,0x00000007,0x0004002b,0x0000000c,0x00000013,0x00000001,0x00040020,
  0x00000014,0x00000001,0x00000008,0x0004003b,0x00000014,0x00000015,0x00000001,0x00040020,
  0x00000017,0x00000003,0x00000008,0x0003001e,0x00000019,0x00000007,0x00040020,0x0000001a,
  0x00000003,0x00000019,0x0004003b,0x0000001a,0x0000001b,0x00000003,0x0004003b,0x00000014,
  0x0000001c,0x00000001,0x0004001e,0x0000001e,0x00000008,0x00000008,0x00040020,0x0000001f,
  0x00000009,0x0000001e,0x0004003b,0x0000001f,0x00000020,0x00000009,0x00040020,0x00000021,
  0x00000009,0x00000008,0x0004002b,0x00000006,0x00000028,0x00000000,0x0004002b,0x00000006,
  0x00000029,0x3f800000,0x00050036,0x00000002,0x00000004,0x00000000,0x00000003,0x000200f8,
  0x00000005,0x0004003d,0x00000007,0x00000010,0x0000000f,0x00050041,0x00000011,0x00000012,
  0x0000000b,0x0000000d,0x0003003e,0x00000012,0x00000010,0x0004003d,0x00000008,0x00000016,
  0x00000015,0x00050041,0x00000017,0x00000018,0x0000000b,0x00000013,0x0003003e,0x00000018,
  0x00000016,0x0004003d,0x00000008,0x0000001d,0x0000001c,0x00050041,0x00000021,0x00000022,
  0x00000020,0x0000000d,0x0004003d,0x00000008,0x00000023,0x00000022,0x00050085,0x00000008,
  0x00000024,0x0000001d,0x00000023,0x00050041,0x00000021,0x00000025,0x00000020,0x00000013,
  0x0004003d,0x00000008,0x00000026,0x00000025,0x00050081,0x00000008,0x00000027,0x00000024,
  0x00000026,0x00050051,0x00000006,0x0000002a,0x00000027,0x00000000,0x00050051,0x00000006,
  0x0000002b,0x00000027,0x00000001,0x00070050,0x00000007,0x0000002c,0x0000002a,0x0000002b,
  0x00000028,0x00000029,0x00050041,0x00000011,0x0000002d,0x0000001b,0x0000000d,0x0003003e,
  0x0000002d,0x0000002c,0x000100fd,0x00010038
];

// # glslangValidator -V -x -o glsl_shader.frag.u32 glsl_shader.frag
/*
#version 450 core
layout(location = 0) out vec4 fColor;
layout(set=0, binding=0) uniform sampler2D sTexture;
layout(location = 0) in struct { vec4 Color; vec2 UV; } In;
void main()
{
    fColor = In.Color * texture(sTexture, In.UV.st);
}
*/
const __GLSL_SHADER_FRAG_SPV: [u32; 193] = [
  0x07230203,0x00010000,0x00080001,0x0000001e,0x00000000,0x00020011,0x00000001,0x0006000b,
  0x00000001,0x4c534c47,0x6474732e,0x3035342e,0x00000000,0x0003000e,0x00000000,0x00000001,
  0x0007000f,0x00000004,0x00000004,0x6e69616d,0x00000000,0x00000009,0x0000000d,0x00030010,
  0x00000004,0x00000007,0x00030003,0x00000002,0x000001c2,0x00040005,0x00000004,0x6e69616d,
  0x00000000,0x00040005,0x00000009,0x6c6f4366,0x0000726f,0x00030005,0x0000000b,0x00000000,
  0x00050006,0x0000000b,0x00000000,0x6f6c6f43,0x00000072,0x00040006,0x0000000b,0x00000001,
  0x00005655,0x00030005,0x0000000d,0x00006e49,0x00050005,0x00000016,0x78655473,0x65727574,
  0x00000000,0x00040047,0x00000009,0x0000001e,0x00000000,0x00040047,0x0000000d,0x0000001e,
  0x00000000,0x00040047,0x00000016,0x00000022,0x00000000,0x00040047,0x00000016,0x00000021,
  0x00000000,0x00020013,0x00000002,0x00030021,0x00000003,0x00000002,0x00030016,0x00000006,
  0x00000020,0x00040017,0x00000007,0x00000006,0x00000004,0x00040020,0x00000008,0x00000003,
  0x00000007,0x0004003b,0x00000008,0x00000009,0x00000003,0x00040017,0x0000000a,0x00000006,
  0x00000002,0x0004001e,0x0000000b,0x00000007,0x0000000a,0x00040020,0x0000000c,0x00000001,
  0x0000000b,0x0004003b,0x0000000c,0x0000000d,0x00000001,0x00040015,0x0000000e,0x00000020,
  0x00000001,0x0004002b,0x0000000e,0x0000000f,0x00000000,0x00040020,0x00000010,0x00000001,
  0x00000007,0x00090019,0x00000013,0x00000006,0x00000001,0x00000000,0x00000000,0x00000000,
  0x00000001,0x00000000,0x0003001b,0x00000014,0x00000013,0x00040020,0x00000015,0x00000000,
  0x00000014,0x0004003b,0x00000015,0x00000016,0x00000000,0x0004002b,0x0000000e,0x00000018,
  0x00000001,0x00040020,0x00000019,0x00000001,0x0000000a,0x00050036,0x00000002,0x00000004,
  0x00000000,0x00000003,0x000200f8,0x00000005,0x00050041,0x00000010,0x00000011,0x0000000d,
  0x0000000f,0x0004003d,0x00000007,0x00000012,0x00000011,0x0004003d,0x00000014,0x00000017,
  0x00000016,0x00050041,0x00000019,0x0000001a,0x0000000d,0x00000018,0x0004003d,0x0000000a,
  0x0000001b,0x0000001a,0x00050057,0x00000007,0x0000001c,0x00000017,0x0000001b,0x00050085,
  0x00000007,0x0000001d,0x00000012,0x0000001c,0x0003003e,0x00000009,0x0000001d,0x000100fd,
  0x00010038
];

/// The ImGUI context.
pub struct HalaImGui {
  pub(crate) vk_ctx: Rc<RefCell<hala_gfx::HalaContext>>,

  vert_shader: std::mem::ManuallyDrop<hala_gfx::HalaShader>,
  frag_shader: std::mem::ManuallyDrop<hala_gfx::HalaShader>,

  font_sampler: std::mem::ManuallyDrop<hala_gfx::HalaSampler>,
  font_descriptor_pool: std::mem::ManuallyDrop<Rc<RefCell<hala_gfx::HalaDescriptorPool>>>,
  font_descriptor_set: std::mem::ManuallyDrop<hala_gfx::HalaDescriptorSet>,

  pipeline: std::mem::ManuallyDrop<hala_gfx::HalaGraphicsPipeline>,

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

      std::mem::ManuallyDrop::drop(&mut self.pipeline);
      std::mem::ManuallyDrop::drop(&mut self.font_descriptor_set);
      std::mem::ManuallyDrop::drop(&mut self.font_descriptor_pool);
      std::mem::ManuallyDrop::drop(&mut self.font_sampler);
      std::mem::ManuallyDrop::drop(&mut self.vert_shader);
      std::mem::ManuallyDrop::drop(&mut self.frag_shader);
    }
    log::debug!("ImGUI context dropped.");
  }

}

/// The implementation of the ImGUI context.
impl HalaImGui {

  /// Create a new ImGUI context.
  pub fn new(vk_ctx: Rc<RefCell<hala_gfx::HalaContext>>) -> Result<Self> {
    let (
      vert_shader,
      frag_shader,
      font_descriptor_pool,
      font_descriptor_set,
      font_sampler,
      pipeline,
    ) = {
      let context = vk_ctx.borrow();

      let vert_code = unsafe {
        std::slice::from_raw_parts(__GLSL_SHADER_VERT_SPV.as_ptr() as *const u8, __GLSL_SHADER_VERT_SPV.len() * 4)
      };
      let vert_shader = hala_gfx::HalaShader::new(
        Rc::clone(&context.logical_device),
        vert_code,
        hala_gfx::HalaShaderStageFlags::VERTEX,
        hala_gfx::HalaRayTracingShaderGroupType::GENERAL,
        "imgui.vert",
      )?;

      let frag_code = unsafe {
        std::slice::from_raw_parts(__GLSL_SHADER_FRAG_SPV.as_ptr() as *const u8, __GLSL_SHADER_FRAG_SPV.len() * 4)
      };
      let frag_shader = hala_gfx::HalaShader::new(
        Rc::clone(&context.logical_device),
        frag_code,
        hala_gfx::HalaShaderStageFlags::FRAGMENT,
        hala_gfx::HalaRayTracingShaderGroupType::GENERAL,
        "imgui.frag",
      )?;

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

      let font_descriptor_set = hala_gfx::HalaDescriptorSet::new_static(
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

      let pipeline = hala_gfx::HalaGraphicsPipeline::new(
        Rc::clone(&context.logical_device),
        &context.swapchain,
        &[&font_descriptor_set.layout],
        &[
          // Position.
          hala_gfx::HalaVertexInputAttributeDescription {
            binding: 0,
            location: 0,
            offset: 0,
            format: hala_gfx::HalaFormat::R32G32_SFLOAT,
          },
          // UV.
          hala_gfx::HalaVertexInputAttributeDescription {
            binding: 0,
            location: 1,
            offset: 8,
            format: hala_gfx::HalaFormat::R32G32_SFLOAT,
          },
          // Color.
          hala_gfx::HalaVertexInputAttributeDescription {
            binding: 0,
            location: 2,
            offset: 16,
            format: hala_gfx::HalaFormat::R8G8B8A8_UNORM,
          },
        ],
        &[
          hala_gfx::HalaVertexInputBindingDescription {
            binding: 0,
            stride: 20,
            input_rate: hala_gfx::HalaVertexInputRate::VERTEX,
          }
        ],
        &[
          hala_gfx::HalaPushConstantRange {
            stage_flags: hala_gfx::HalaShaderStageFlags::VERTEX,
            offset: 0,
            size: 16,
          }
        ],
        hala_gfx::HalaPrimitiveTopology::TRIANGLE_LIST,
        (hala_gfx::HalaBlendFactor::SRC_ALPHA, hala_gfx::HalaBlendFactor::ONE_MINUS_SRC_ALPHA, hala_gfx::HalaBlendOp::ADD),
        (hala_gfx::HalaBlendFactor::ONE, hala_gfx::HalaBlendFactor::ONE_MINUS_SRC_ALPHA, hala_gfx::HalaBlendOp::ADD),
        (1.0, hala_gfx::HalaFrontFace::COUNTER_CLOCKWISE, hala_gfx::HalaCullModeFlags::NONE, hala_gfx::HalaPolygonMode::FILL),
        (false, false, hala_gfx::HalaCompareOp::NEVER),
        &[&vert_shader, &frag_shader],
        None,
        "imgui.pipeline",
      )?;

      (
        std::mem::ManuallyDrop::new(vert_shader),
        std::mem::ManuallyDrop::new(frag_shader),
        std::mem::ManuallyDrop::new(font_descriptor_pool),
        std::mem::ManuallyDrop::new(font_descriptor_set),
        std::mem::ManuallyDrop::new(font_sampler),
        std::mem::ManuallyDrop::new(pipeline),
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
      vert_shader,
      frag_shader,
      font_descriptor_pool,
      font_descriptor_set,
      font_sampler,
      pipeline,
      font_image: None,
      imgui,
    })
  }

  /// Begin the ImGUI frame.
  /// param delta_time: The delta time.
  /// param width: The width of the window.
  /// param height: The height of the window.
  /// return: The result.
  pub fn begin_frame(&mut self, delta_time: f64, width: u32, height: u32) -> Result<()> {
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

  /// End the ImGUI frame.
  /// return: The result.
  pub fn end_frame(&self) -> Result<()> {
    unsafe {
      ImGui_Render();
    }

    Ok(())
  }

  /// Draw the ImGUI.
  /// param index: The index.
  /// param command_buffers: The command buffers.
  /// return: The result.
  pub fn draw(&self, index: usize, command_buffers: &hala_gfx::HalaCommandBufferSet) -> core::result::Result<(), hala_gfx::HalaGfxError> {
    let draw_data = unsafe {
      let draw_data = ImGui_GetDrawData();
      let is_minimized = (*draw_data).DisplaySize.x <= 0.0 || (*draw_data).DisplaySize.y <= 0.0;
      if !is_minimized {
        draw_data
      } else {
        null()
      }
    };
    if draw_data.is_null() {
      return core::result::Result::Ok(());
    }

    command_buffers.bind_graphics_pipeline(index, &self.pipeline);
    command_buffers.bind_graphics_descriptor_sets(
      index,
      &self.pipeline,
      0,
      &[&(*self.font_descriptor_set)],
      &[],
    );

    core::result::Result::Ok(())
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