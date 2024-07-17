use std::{
  cell::RefCell,
  rc::Rc
};

use anyhow::{Ok, Result};

use imgui::internal::{RawCast, RawWrapper};
use winit::event::MouseButton;
use winit::keyboard::{
  PhysicalKey,
  KeyCode,
};

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
  vertex_buffers: Vec<Option<hala_gfx::HalaBuffer>>,
  index_buffers: Vec<Option<hala_gfx::HalaBuffer>>,

  imgui: std::mem::ManuallyDrop<imgui::Context>,
}

/// The implementation of the drop trait for the ImGUI context.
impl Drop for HalaImGui {

  /// Drop the ImGUI context.
  fn drop(&mut self) {
    self.font_image = None;

    unsafe {
      std::mem::ManuallyDrop::drop(&mut self.imgui);
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

  pub fn to_button(btncode: MouseButton) -> Option<imgui::MouseButton> {
    let btn = match btncode {
      MouseButton::Left => imgui::MouseButton::Left,
      MouseButton::Right => imgui::MouseButton::Right,
      MouseButton::Middle => imgui::MouseButton::Middle,
      MouseButton::Other(x) if x < imgui::MouseButton::COUNT as u16 => {
        match x {
          0 => imgui::MouseButton::Left,
          1 => imgui::MouseButton::Right,
          2 => imgui::MouseButton::Middle,
          3 => imgui::MouseButton::Extra1,
          4 => imgui::MouseButton::Extra2,
          _ => return None,
        }
      }
      _ => return None,
    };
    Some(btn)
  }
  pub fn to_key(phys_key: PhysicalKey) -> Option<imgui::Key> {
    let key = match phys_key {
      PhysicalKey::Code(code) => match code {
        KeyCode::Tab => imgui::Key::Tab,
        KeyCode::ArrowLeft => imgui::Key::LeftArrow,
        KeyCode::ArrowRight => imgui::Key::RightArrow,
        KeyCode::ArrowUp => imgui::Key::UpArrow,
        KeyCode::ArrowDown => imgui::Key::DownArrow,
        KeyCode::PageUp => imgui::Key::PageUp,
        KeyCode::PageDown => imgui::Key::PageDown,
        KeyCode::Home => imgui::Key::Home,
        KeyCode::End => imgui::Key::End,
        KeyCode::Insert => imgui::Key::Insert,
        KeyCode::Delete => imgui::Key::Delete,
        KeyCode::Backspace => imgui::Key::Backspace,
        KeyCode::Space => imgui::Key::Space,
        KeyCode::Enter => imgui::Key::Enter,
        KeyCode::Escape => imgui::Key::Escape,
        KeyCode::ControlLeft => imgui::Key::LeftCtrl,
        KeyCode::ShiftLeft => imgui::Key::LeftShift,
        KeyCode::AltLeft => imgui::Key::LeftAlt,
        KeyCode::SuperLeft => imgui::Key::LeftSuper,
        KeyCode::ControlRight => imgui::Key::RightCtrl,
        KeyCode::ShiftRight => imgui::Key::RightShift,
        KeyCode::AltRight => imgui::Key::RightAlt,
        KeyCode::SuperRight => imgui::Key::RightSuper,
        KeyCode::Digit0 => imgui::Key::Alpha0,
        KeyCode::Digit1 => imgui::Key::Alpha1,
        KeyCode::Digit2 => imgui::Key::Alpha2,
        KeyCode::Digit3 => imgui::Key::Alpha3,
        KeyCode::Digit4 => imgui::Key::Alpha4,
        KeyCode::Digit5 => imgui::Key::Alpha5,
        KeyCode::Digit6 => imgui::Key::Alpha6,
        KeyCode::Digit7 => imgui::Key::Alpha7,
        KeyCode::Digit8 => imgui::Key::Alpha8,
        KeyCode::Digit9 => imgui::Key::Alpha9,
        KeyCode::KeyA => imgui::Key::A,
        KeyCode::KeyB => imgui::Key::B,
        KeyCode::KeyC => imgui::Key::C,
        KeyCode::KeyD => imgui::Key::D,
        KeyCode::KeyE => imgui::Key::E,
        KeyCode::KeyF => imgui::Key::F,
        KeyCode::KeyG => imgui::Key::G,
        KeyCode::KeyH => imgui::Key::H,
        KeyCode::KeyI => imgui::Key::I,
        KeyCode::KeyJ => imgui::Key::J,
        KeyCode::KeyK => imgui::Key::K,
        KeyCode::KeyL => imgui::Key::L,
        KeyCode::KeyM => imgui::Key::M,
        KeyCode::KeyN => imgui::Key::N,
        KeyCode::KeyO => imgui::Key::O,
        KeyCode::KeyP => imgui::Key::P,
        KeyCode::KeyQ => imgui::Key::Q,
        KeyCode::KeyR => imgui::Key::R,
        KeyCode::KeyS => imgui::Key::S,
        KeyCode::KeyT => imgui::Key::T,
        KeyCode::KeyU => imgui::Key::U,
        KeyCode::KeyV => imgui::Key::V,
        KeyCode::KeyW => imgui::Key::W,
        KeyCode::KeyX => imgui::Key::X,
        KeyCode::KeyY => imgui::Key::Y,
        KeyCode::KeyZ => imgui::Key::Z,
        KeyCode::F1 => imgui::Key::F1,
        KeyCode::F2 => imgui::Key::F2,
        KeyCode::F3 => imgui::Key::F3,
        KeyCode::F4 => imgui::Key::F4,
        KeyCode::F5 => imgui::Key::F5,
        KeyCode::F6 => imgui::Key::F6,
        KeyCode::F7 => imgui::Key::F7,
        KeyCode::F8 => imgui::Key::F8,
        KeyCode::F9 => imgui::Key::F9,
        KeyCode::F10 => imgui::Key::F10,
        KeyCode::F11 => imgui::Key::F11,
        KeyCode::F12 => imgui::Key::F12,
        KeyCode::Quote => imgui::Key::Apostrophe,
        KeyCode::Comma => imgui::Key::Comma,
        KeyCode::Minus => imgui::Key::Minus,
        KeyCode::Period => imgui::Key::Period,
        KeyCode::Slash => imgui::Key::Slash,
        KeyCode::Semicolon => imgui::Key::Semicolon,
        KeyCode::Equal => imgui::Key::Equal,
        KeyCode::BracketLeft => imgui::Key::LeftBracket,
        KeyCode::Backslash => imgui::Key::Backslash,
        KeyCode::BracketRight => imgui::Key::RightBracket,
        KeyCode::Backquote => imgui::Key::GraveAccent,
        KeyCode::CapsLock => imgui::Key::CapsLock,
        KeyCode::ScrollLock => imgui::Key::ScrollLock,
        KeyCode::NumLock => imgui::Key::NumLock,
        KeyCode::PrintScreen => imgui::Key::PrintScreen,
        KeyCode::Pause => imgui::Key::Pause,
        KeyCode::Numpad0 => imgui::Key::Keypad0,
        KeyCode::Numpad1 => imgui::Key::Keypad1,
        KeyCode::Numpad2 => imgui::Key::Keypad2,
        KeyCode::Numpad3 => imgui::Key::Keypad3,
        KeyCode::Numpad4 => imgui::Key::Keypad4,
        KeyCode::Numpad5 => imgui::Key::Keypad5,
        KeyCode::Numpad6 => imgui::Key::Keypad6,
        KeyCode::Numpad7 => imgui::Key::Keypad7,
        KeyCode::Numpad8 => imgui::Key::Keypad8,
        KeyCode::Numpad9 => imgui::Key::Keypad9,
        KeyCode::NumpadDecimal => imgui::Key::KeypadDecimal,
        KeyCode::NumpadDivide => imgui::Key::KeypadDivide,
        KeyCode::NumpadMultiply => imgui::Key::KeypadMultiply,
        KeyCode::NumpadSubtract => imgui::Key::KeypadSubtract,
        KeyCode::NumpadAdd => imgui::Key::KeypadAdd,
        KeyCode::NumpadEnter => imgui::Key::KeypadEnter,
        KeyCode::NumpadEqual => imgui::Key::KeypadEqual,
        KeyCode::NumpadBackspace => imgui::Key::Backspace,
        _ => return None,
      },
      PhysicalKey::Unidentified(_) => return None,
    };
    Some(key)
  }

  /// Create a new ImGUI context.
  /// param vk_ctx The Vulkan context.
  /// param enable_ini Whether to enable the INI file.
  /// return The result of the creation.
  pub fn new(vk_ctx: Rc<RefCell<hala_gfx::HalaContext>>, enable_ini: bool) -> Result<Self> {
    let (
      vert_shader,
      frag_shader,
      font_descriptor_pool,
      font_descriptor_set,
      font_sampler,
      pipeline,
      num_of_images,
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
        "imgui.vert.spv",
      )?;

      let frag_code = unsafe {
        std::slice::from_raw_parts(__GLSL_SHADER_FRAG_SPV.as_ptr() as *const u8, __GLSL_SHADER_FRAG_SPV.len() * 4)
      };
      let frag_shader = hala_gfx::HalaShader::new(
        Rc::clone(&context.logical_device),
        frag_code,
        hala_gfx::HalaShaderStageFlags::FRAGMENT,
        hala_gfx::HalaRayTracingShaderGroupType::GENERAL,
        "imgui.frag.spv",
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
            hala_gfx::HalaDescriptorSetLayoutBinding {
              binding_index: 0,
              descriptor_type: hala_gfx::HalaDescriptorType::COMBINED_IMAGE_SAMPLER,
              descriptor_count: 1,
              stage_flags: hala_gfx::HalaShaderStageFlags::FRAGMENT,
              binding_flags: hala_gfx::HalaDescriptorBindingFlags::PARTIALLY_BOUND,
            },
          ],
          "imgui_font.descsetlayout",
        )?,
        0,
        "imgui_font.descset",
      )?;

      let pipeline = hala_gfx::HalaGraphicsPipeline::new(
        Rc::clone(&context.logical_device),
        &context.swapchain,
        &[&font_descriptor_set.layout],
        hala_gfx::HalaPipelineCreateFlags::default(),
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
        &hala_gfx::HalaBlendState::new(hala_gfx::HalaBlendFactor::SRC_ALPHA, hala_gfx::HalaBlendFactor::ONE_MINUS_SRC_ALPHA, hala_gfx::HalaBlendOp::ADD),
        &hala_gfx::HalaBlendState::new(hala_gfx::HalaBlendFactor::ONE, hala_gfx::HalaBlendFactor::ONE_MINUS_SRC_ALPHA, hala_gfx::HalaBlendOp::ADD),
        &hala_gfx::HalaRasterizerState::new(hala_gfx::HalaFrontFace::COUNTER_CLOCKWISE, hala_gfx::HalaCullModeFlags::NONE, hala_gfx::HalaPolygonMode::FILL, 1.0),
        &hala_gfx::HalaDepthState::new(false, false, hala_gfx::HalaCompareOp::NEVER),
        None,
        &[&vert_shader, &frag_shader],
        &[hala_gfx::HalaDynamicState::VIEWPORT, hala_gfx::HalaDynamicState::SCISSOR],
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
        context.swapchain.num_of_images,
      )
    };

    // Initialize ImGUI.
    let mut imgui = imgui::Context::create();
    imgui.style_mut().use_dark_colors();
    if !enable_ini {
      imgui.set_ini_filename(None);
    }
    imgui.io_mut().config_flags = imgui::ConfigFlags::NAV_ENABLE_KEYBOARD | imgui::ConfigFlags::NAV_ENABLE_GAMEPAD;

    let mut vertex_buffers = Vec::with_capacity(num_of_images);
    vertex_buffers.resize_with(num_of_images, || None);
    let mut index_buffers = Vec::with_capacity(num_of_images);
    index_buffers.resize_with(num_of_images, || None);

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
      vertex_buffers,
      index_buffers,
      imgui: std::mem::ManuallyDrop::new(imgui),
    })
  }

  /// Whether the imgui wants to capture the mouse.
  /// When true, imgui will use the mouse inputs, so do not dispatch them to your main
  pub fn want_capture_mouse(&self) -> bool {
    self.imgui.io().want_capture_mouse
  }

  /// Whether the imgui wants to capture the keyboard.
  /// When true, imgui will use the keyboard inputs, so do not dispatch them to your main
  pub fn want_capture_keyboard(&self) -> bool {
    self.imgui.io().want_capture_keyboard
  }

  /// Begin the ImGUI frame.
  /// param delta_time: The delta time.
  /// param width: The width of the window.
  /// param height: The height of the window.
  /// return: The result.
  pub fn begin_frame<F>(&mut self, delta_time: f64, width: u32, height: u32, mut ui_fn: F) -> Result<()>
    where F: FnMut(&mut imgui::Ui)
  {
    self.imgui.io_mut().delta_time = delta_time as f32;
    self.imgui.io_mut().display_size = [width as f32, height as f32];
    self.imgui.io_mut().display_framebuffer_scale = [1.0, 1.0];

    if self.font_image.is_none() {
      self.create_fonts_texture()?;
    }

    ui_fn(self.imgui.new_frame());

    Ok(())
  }

  /// End the ImGUI frame.
  /// return: The result.
  pub fn end_frame(&mut self) -> Result<()> {
    self.imgui.render();

    Ok(())
  }

  /// Draw the ImGUI.
  /// param index: The index.
  /// param command_buffers: The command buffers.
  /// return: The result.
  pub fn draw(&mut self, index: usize, command_buffers: &hala_gfx::HalaCommandBufferSet) -> core::result::Result<(), hala_gfx::HalaGfxError> {
    // Get draw data.
    let draw_data = unsafe {
      let draw_data = imgui::sys::igGetDrawData() as *mut imgui::DrawData;
      if draw_data.is_null() {
        return core::result::Result::Ok(());
      }
      &*draw_data
    };
    let is_minimized = draw_data.display_size[0] <= 0.0 || draw_data.display_size[1] <= 0.0;
    let total_vtx_count = draw_data.total_vtx_count;
    let total_idx_count = draw_data.total_idx_count;
    if is_minimized || total_vtx_count == 0 || total_idx_count == 0 {
      return core::result::Result::Ok(());
    }

    let context = self.vk_ctx.borrow();

    // Create or resize the vertex/index buffers.
    let vertex_buffer_size = total_vtx_count as u64 * std::mem::size_of::<imgui::DrawVert>() as u64;
    let vertex_buffer = match self.vertex_buffers[index] {
      Some(ref buffer) if buffer.size >= vertex_buffer_size => buffer,
      _ => {
        self.vertex_buffers[index] = Some(hala_gfx::HalaBuffer::new(
          Rc::clone(&context.logical_device),
          vertex_buffer_size,
          hala_gfx::HalaBufferUsageFlags::VERTEX_BUFFER,
          hala_gfx::HalaMemoryLocation::CpuToGpu,
          &format!("imgui_vertex_{}.buffer", index),
        )?);
        self.vertex_buffers[index].as_ref().unwrap()
      },
    };
    let index_buffer_size = total_idx_count as u64 * std::mem::size_of::<imgui::DrawIdx>() as u64;
    let index_buffer = match self.index_buffers[index] {
      Some(ref buffer) if buffer.size >= index_buffer_size => buffer,
      _ => {
        self.index_buffers[index] = Some(hala_gfx::HalaBuffer::new(
          Rc::clone(&context.logical_device),
          index_buffer_size,
          hala_gfx::HalaBufferUsageFlags::INDEX_BUFFER,
          hala_gfx::HalaMemoryLocation::CpuToGpu,
          &format!("imgui_index_{}.buffer", index),
        )?);
        self.index_buffers[index].as_ref().unwrap()
      },
    };

    // Fill the vertex/index buffers.
    let mut vtx_offset = 0;
    let mut idx_offset = 0;
    for cmd_list in draw_data.draw_lists() {
      let vtx_buffer = cmd_list.vtx_buffer();
      let vtx_size = std::mem::size_of_val(vtx_buffer);
      vertex_buffer.update_memory(
        vtx_offset,
        vtx_buffer
      )?;
      vtx_offset += vtx_size;

      let idx_buffer = cmd_list.idx_buffer();
      let idx_size = std::mem::size_of_val(idx_buffer);
      index_buffer.update_memory(
        idx_offset,
        idx_buffer,
      )?;
      idx_offset += idx_size;
    }

    // Will project scissor/clipping rectangles into framebuffer space
    let (clip_off, clip_scale, fb_size) = (
      draw_data.display_pos,        // (0,0) unless using multi-viewports
      draw_data.framebuffer_scale,  // (1,1) unless using retina display which are often (2,2)
      self.imgui.io().display_size,
    );

    // Bind pipeline.
    command_buffers.bind_graphics_pipeline(index, &self.pipeline);

    // Bind vertex/index buffers.
    command_buffers.bind_vertex_buffers(index, 0, &[vertex_buffer], &[0]);
    command_buffers.bind_index_buffers(index, &[index_buffer], &[0], hala_gfx::HalaIndexType::UINT16);

    // Set viewport.
    command_buffers.set_viewport(
      index,
      0,
      &[(
        0.0,
        0.0,
        fb_size[0],
        fb_size[1],
        0.0,
        1.0,
      )],
    );

    // Setup scale and translation:
    // Our visible imgui space lies from draw_data->DisplayPps (top left) to draw_data->DisplayPos+data_data->DisplaySize (bottom right). DisplayPos is (0,0) for single viewport apps.
    let scale = [
      2.0 / fb_size[0],
      2.0 / fb_size[1],
    ];
    let translate = [
      -1.0 - clip_off[0] * scale[0],
      -1.0 - clip_off[1] * scale[1],
    ];
    command_buffers.push_constants_f32(
      index,
      self.pipeline.layout,
      hala_gfx::HalaShaderStageFlags::VERTEX,
      0,
      &scale,
    );
    command_buffers.push_constants_f32(
      index,
      self.pipeline.layout,
      hala_gfx::HalaShaderStageFlags::VERTEX,
      std::mem::size_of_val(&scale) as u32,
      &translate,
    );

    // Render command list.
    unsafe {
      let mut vtx_offset = 0;
      let mut idx_offset = 0;
      for cmd_list in draw_data.draw_lists() {
        for cmd in cmd_list.commands() {
          match cmd {
            imgui::DrawCmd::ResetRenderState => {
              // TODO: Reset render state.
            },
            imgui::DrawCmd::RawCallback { callback, raw_cmd } => {
              callback(cmd_list.raw(), raw_cmd);
            },
            imgui::DrawCmd::Elements { count, cmd_params } => {
              // Project scissor/clipping rectangles into framebuffer space.
              let mut clip_min = [
                (cmd_params.clip_rect[0] - clip_off[0]) * clip_scale[0],
                (cmd_params.clip_rect[1] - clip_off[1]) * clip_scale[1],
              ];
              let mut clip_max = [
                (cmd_params.clip_rect[2] - clip_off[0]) * clip_scale[0],
                (cmd_params.clip_rect[3] - clip_off[1]) * clip_scale[1],
              ];

              // Clamp to viewport as vkCmdSetScissor() won't accept values that are off bounds.
              if clip_min[0] < 0.0 { clip_min[0] = 0.0; }
              if clip_min[1] < 0.0 { clip_min[1] = 0.0; }
              if clip_max[0] > fb_size[0] { clip_max[0] = fb_size[0]; }
              if clip_max[1] > fb_size[1] { clip_max[1] = fb_size[1]; }
              if clip_max[0] <= clip_min[0] || clip_max[1] <= clip_min[1] {
                continue;
              }

              // Apply scissor/clipping rectangle.
              command_buffers.set_scissor(
                index,
                0,
                &[
                  (clip_min[0] as i32, clip_min[1] as i32, (clip_max[0] - clip_min[0]) as u32, (clip_max[1] - clip_min[1]) as u32)
                ],
              );

              // Bind DescriptorSet with font or user texture.
              command_buffers.bind_graphics_descriptor_sets(
                index,
                &self.pipeline,
                0,
                &[&(*self.font_descriptor_set)],
                &[],
              );

              // Draw.
              command_buffers.draw_indexed(
                index,
                count as u32,
                1,
                cmd_params.idx_offset as u32 + idx_offset,
                cmd_params.vtx_offset as i32 + vtx_offset,
                0,
              );
            }
          }
        }

        idx_offset += cmd_list.idx_buffer().len() as u32;
        vtx_offset += cmd_list.vtx_buffer().len() as i32;
      }
    }

    // Note: at this point both vkCmdSetViewport() and vkCmdSetScissor() have been called.
    // Our last values will leak into user/application rendering.
    command_buffers.set_scissor(
      index,
      0,
      &[
        (0, 0, fb_size[0] as u32, fb_size[1] as u32)
      ],
    );

    core::result::Result::Ok(())
  }

  /// Whether any mouse button is down.
  /// return: Whether any mouse button is down.
  pub fn is_any_mouse_down(&self) -> bool {
    unsafe {
      imgui::sys::igIsAnyMouseDown()
    }
  }

  /// Get the display scale.
  /// return: The display scale.
  pub fn get_display_framebuffer_scale(&self) -> [f32; 2] {
    self.imgui.io().display_framebuffer_scale
  }

  /// Get font size.
  /// return: The font size.
  pub fn get_font_size(&self) -> f32 {
    unsafe {
      imgui::sys::igGetFontSize()
    }
  }

  /// Add a key event.
  /// param key: The key.
  /// param is_down: Whether the key is down.
  pub fn add_key_event(&mut self, key: imgui::Key, is_down: bool) {
    self.imgui.io_mut().add_key_event(key, is_down)
  }

  /// Add a input character.
  /// param char: The character.
  pub fn add_input_character(&mut self, char: u32) {
    self.imgui.io_mut().add_input_character(char::from_u32(char).unwrap())
  }

  /// Add a focus event.
  /// param is_focused: Whether the window is focused.
  pub fn add_focus_event(&mut self, is_focused: bool) {
    unsafe {
      let io = self.imgui.io_mut().raw_mut();
      imgui::sys::ImGuiIO_AddFocusEvent(io, is_focused)
    }
  }

  /// Add a mouse position event.
  /// param x: The x position.
  /// param y: The y position.
  pub fn add_mouse_pos_event(&mut self, x: f32, y: f32) {
    self.imgui.io_mut().add_mouse_pos_event([x, y])
  }

  /// Add a mouse button event.
  /// param button: The button.
  /// param is_down: Whether the button is down.
  pub fn add_mouse_button_event(&mut self, button: imgui::MouseButton, is_down: bool) {
    self.imgui.io_mut().add_mouse_button_event(button, is_down)
  }

  /// Add a mouse wheel event.
  /// param h: The horizontal delta value.
  /// param v: The vertical delta value.
  pub fn add_mouse_wheel_event(&mut self, h: f32, v: f32) {
    self.imgui.io_mut().add_mouse_wheel_event([h, v])
  }

  /// Create the fonts texture.
  fn create_fonts_texture(&mut self) -> Result<()> {
    let context = self.vk_ctx.borrow();

    let font_texture = self.imgui.fonts().build_rgba32_texture();
    let upload_size = font_texture.data.len();
    let width = font_texture.width;
    let height = font_texture.height;
    let pixels = font_texture.data;

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
      upload_size as u64,
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
    font_image.update_gpu_memory_with_buffer(
      pixels,
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

    Ok(())
  }

}