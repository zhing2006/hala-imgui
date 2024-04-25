use std::{
  cell::RefCell,
  ptr::{null_mut, null},
  rc::Rc
};

use anyhow::{Ok, Result};

use winit::event::MouseButton;
use winit::keyboard::{
  PhysicalKey,
  KeyCode,
};

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
  vertex_buffers: Vec<Option<hala_gfx::HalaBuffer>>,
  index_buffers: Vec<Option<hala_gfx::HalaBuffer>>,

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

  pub fn to_button(btncode: MouseButton) -> Option<ImGuiMouseButton> {
    let btn = match btncode {
      MouseButton::Left => ImGuiMouseButton_::ImGuiMouseButton_Left.0,
      MouseButton::Right => ImGuiMouseButton_::ImGuiMouseButton_Right.0,
      MouseButton::Middle => ImGuiMouseButton_::ImGuiMouseButton_Middle.0,
      MouseButton::Other(x) if x < ImGuiMouseButton_::ImGuiMouseButton_COUNT.0 as u16 => {
        ImGuiMouseButton_(x as i32).0
      }
      _ => return None,
    };
    Some(btn)
  }
  pub fn to_key(phys_key: PhysicalKey) -> Option<ImGuiKey> {
    let key = match phys_key {
      PhysicalKey::Code(code) => match code {
        KeyCode::Tab => ImGuiKey::ImGuiKey_Tab,
        KeyCode::ArrowLeft => ImGuiKey::ImGuiKey_LeftArrow,
        KeyCode::ArrowRight => ImGuiKey::ImGuiKey_RightArrow,
        KeyCode::ArrowUp => ImGuiKey::ImGuiKey_UpArrow,
        KeyCode::ArrowDown => ImGuiKey::ImGuiKey_DownArrow,
        KeyCode::PageUp => ImGuiKey::ImGuiKey_PageUp,
        KeyCode::PageDown => ImGuiKey::ImGuiKey_PageDown,
        KeyCode::Home => ImGuiKey::ImGuiKey_Home,
        KeyCode::End => ImGuiKey::ImGuiKey_End,
        KeyCode::Insert => ImGuiKey::ImGuiKey_Insert,
        KeyCode::Delete => ImGuiKey::ImGuiKey_Delete,
        KeyCode::Backspace => ImGuiKey::ImGuiKey_Backspace,
        KeyCode::Space => ImGuiKey::ImGuiKey_Space,
        KeyCode::Enter => ImGuiKey::ImGuiKey_Enter,
        KeyCode::Escape => ImGuiKey::ImGuiKey_Escape,
        KeyCode::ControlLeft => ImGuiKey::ImGuiKey_LeftCtrl,
        KeyCode::ShiftLeft => ImGuiKey::ImGuiKey_LeftShift,
        KeyCode::AltLeft => ImGuiKey::ImGuiKey_LeftAlt,
        KeyCode::SuperLeft => ImGuiKey::ImGuiKey_LeftSuper,
        KeyCode::ControlRight => ImGuiKey::ImGuiKey_RightCtrl,
        KeyCode::ShiftRight => ImGuiKey::ImGuiKey_RightShift,
        KeyCode::AltRight => ImGuiKey::ImGuiKey_RightAlt,
        KeyCode::SuperRight => ImGuiKey::ImGuiKey_RightSuper,
        KeyCode::Digit0 => ImGuiKey::ImGuiKey_0,
        KeyCode::Digit1 => ImGuiKey::ImGuiKey_1,
        KeyCode::Digit2 => ImGuiKey::ImGuiKey_2,
        KeyCode::Digit3 => ImGuiKey::ImGuiKey_3,
        KeyCode::Digit4 => ImGuiKey::ImGuiKey_4,
        KeyCode::Digit5 => ImGuiKey::ImGuiKey_5,
        KeyCode::Digit6 => ImGuiKey::ImGuiKey_6,
        KeyCode::Digit7 => ImGuiKey::ImGuiKey_7,
        KeyCode::Digit8 => ImGuiKey::ImGuiKey_8,
        KeyCode::Digit9 => ImGuiKey::ImGuiKey_9,
        KeyCode::KeyA => ImGuiKey::ImGuiKey_A,
        KeyCode::KeyB => ImGuiKey::ImGuiKey_B,
        KeyCode::KeyC => ImGuiKey::ImGuiKey_C,
        KeyCode::KeyD => ImGuiKey::ImGuiKey_D,
        KeyCode::KeyE => ImGuiKey::ImGuiKey_E,
        KeyCode::KeyF => ImGuiKey::ImGuiKey_F,
        KeyCode::KeyG => ImGuiKey::ImGuiKey_G,
        KeyCode::KeyH => ImGuiKey::ImGuiKey_H,
        KeyCode::KeyI => ImGuiKey::ImGuiKey_I,
        KeyCode::KeyJ => ImGuiKey::ImGuiKey_J,
        KeyCode::KeyK => ImGuiKey::ImGuiKey_K,
        KeyCode::KeyL => ImGuiKey::ImGuiKey_L,
        KeyCode::KeyM => ImGuiKey::ImGuiKey_M,
        KeyCode::KeyN => ImGuiKey::ImGuiKey_N,
        KeyCode::KeyO => ImGuiKey::ImGuiKey_O,
        KeyCode::KeyP => ImGuiKey::ImGuiKey_P,
        KeyCode::KeyQ => ImGuiKey::ImGuiKey_Q,
        KeyCode::KeyR => ImGuiKey::ImGuiKey_R,
        KeyCode::KeyS => ImGuiKey::ImGuiKey_S,
        KeyCode::KeyT => ImGuiKey::ImGuiKey_T,
        KeyCode::KeyU => ImGuiKey::ImGuiKey_U,
        KeyCode::KeyV => ImGuiKey::ImGuiKey_V,
        KeyCode::KeyW => ImGuiKey::ImGuiKey_W,
        KeyCode::KeyX => ImGuiKey::ImGuiKey_X,
        KeyCode::KeyY => ImGuiKey::ImGuiKey_Y,
        KeyCode::KeyZ => ImGuiKey::ImGuiKey_Z,
        KeyCode::F1 => ImGuiKey::ImGuiKey_F1,
        KeyCode::F2 => ImGuiKey::ImGuiKey_F2,
        KeyCode::F3 => ImGuiKey::ImGuiKey_F3,
        KeyCode::F4 => ImGuiKey::ImGuiKey_F4,
        KeyCode::F5 => ImGuiKey::ImGuiKey_F5,
        KeyCode::F6 => ImGuiKey::ImGuiKey_F6,
        KeyCode::F7 => ImGuiKey::ImGuiKey_F7,
        KeyCode::F8 => ImGuiKey::ImGuiKey_F8,
        KeyCode::F9 => ImGuiKey::ImGuiKey_F9,
        KeyCode::F10 => ImGuiKey::ImGuiKey_F10,
        KeyCode::F11 => ImGuiKey::ImGuiKey_F11,
        KeyCode::F12 => ImGuiKey::ImGuiKey_F12,
        KeyCode::Quote => ImGuiKey::ImGuiKey_Apostrophe,
        KeyCode::Comma => ImGuiKey::ImGuiKey_Comma,
        KeyCode::Minus => ImGuiKey::ImGuiKey_Minus,
        KeyCode::Period => ImGuiKey::ImGuiKey_Period,
        KeyCode::Slash => ImGuiKey::ImGuiKey_Slash,
        KeyCode::Semicolon => ImGuiKey::ImGuiKey_Semicolon,
        KeyCode::Equal => ImGuiKey::ImGuiKey_Equal,
        KeyCode::BracketLeft => ImGuiKey::ImGuiKey_LeftBracket,
        KeyCode::Backslash => ImGuiKey::ImGuiKey_Backslash,
        KeyCode::BracketRight => ImGuiKey::ImGuiKey_RightBracket,
        KeyCode::Backquote => ImGuiKey::ImGuiKey_GraveAccent,
        KeyCode::CapsLock => ImGuiKey::ImGuiKey_CapsLock,
        KeyCode::ScrollLock => ImGuiKey::ImGuiKey_ScrollLock,
        KeyCode::NumLock => ImGuiKey::ImGuiKey_NumLock,
        KeyCode::PrintScreen => ImGuiKey::ImGuiKey_PrintScreen,
        KeyCode::Pause => ImGuiKey::ImGuiKey_Pause,
        KeyCode::Numpad0 => ImGuiKey::ImGuiKey_Keypad0,
        KeyCode::Numpad1 => ImGuiKey::ImGuiKey_Keypad1,
        KeyCode::Numpad2 => ImGuiKey::ImGuiKey_Keypad2,
        KeyCode::Numpad3 => ImGuiKey::ImGuiKey_Keypad3,
        KeyCode::Numpad4 => ImGuiKey::ImGuiKey_Keypad4,
        KeyCode::Numpad5 => ImGuiKey::ImGuiKey_Keypad5,
        KeyCode::Numpad6 => ImGuiKey::ImGuiKey_Keypad6,
        KeyCode::Numpad7 => ImGuiKey::ImGuiKey_Keypad7,
        KeyCode::Numpad8 => ImGuiKey::ImGuiKey_Keypad8,
        KeyCode::Numpad9 => ImGuiKey::ImGuiKey_Keypad9,
        KeyCode::NumpadDecimal => ImGuiKey::ImGuiKey_KeypadDecimal,
        KeyCode::NumpadDivide => ImGuiKey::ImGuiKey_KeypadDivide,
        KeyCode::NumpadMultiply => ImGuiKey::ImGuiKey_KeypadMultiply,
        KeyCode::NumpadSubtract => ImGuiKey::ImGuiKey_KeypadSubtract,
        KeyCode::NumpadAdd => ImGuiKey::ImGuiKey_KeypadAdd,
        KeyCode::NumpadEnter => ImGuiKey::ImGuiKey_KeypadEnter,
        KeyCode::NumpadEqual => ImGuiKey::ImGuiKey_KeypadEqual,
        KeyCode::NumpadBackspace => ImGuiKey::ImGuiKey_Backspace,
        _ => return None,
      },
      PhysicalKey::Unidentified(_) => return None,
    };
    Some(key)
  }

  /// Create a new ImGUI context.
  pub fn new(vk_ctx: Rc<RefCell<hala_gfx::HalaContext>>) -> Result<Self> {
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
  pub fn draw(&mut self, index: usize, command_buffers: &hala_gfx::HalaCommandBufferSet) -> core::result::Result<(), hala_gfx::HalaGfxError> {
    // Get draw data.
    let (
      draw_data,
      total_vtx_count,
      total_idx_count,
    ) = unsafe {
      let draw_data = ImGui_GetDrawData();
      let is_minimized = (*draw_data).DisplaySize.x <= 0.0 || (*draw_data).DisplaySize.y <= 0.0;
      if !is_minimized {
        (draw_data as *const ImDrawData, (*draw_data).TotalVtxCount, (*draw_data).TotalIdxCount)
      } else {
        (null(), 0, 0)
      }
    };
    if draw_data.is_null() || total_vtx_count == 0 || total_idx_count == 0 {
      return core::result::Result::Ok(());
    }

    let context = self.vk_ctx.borrow();

    // Create or resize the vertex/index buffers.
    let vertex_buffer_size = total_vtx_count as u64 * std::mem::size_of::<ImDrawVert>() as u64;
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
    let index_buffer_size = total_idx_count as u64 * std::mem::size_of::<ImDrawIdx>() as u64;
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
    unsafe {
      let mut vtx_offset = 0;
      let mut idx_offset = 0;
      for n in 0..(*draw_data).CmdListsCount {
        let cmd_list = (*draw_data).CmdLists[n as usize];

        let vtx_size = (*cmd_list).VtxBuffer.Size as usize * std::mem::size_of::<ImDrawVert>();
        vertex_buffer.update_memory_raw(
          vtx_offset,
          (*cmd_list).VtxBuffer.Data as *const u8,
          vtx_size,
        )?;
        vtx_offset += vtx_size;

        let idx_size = (*cmd_list).IdxBuffer.Size as usize * std::mem::size_of::<ImDrawIdx>();
        index_buffer.update_memory_raw(
          idx_offset,
          (*cmd_list).IdxBuffer.Data as *const u8,
          idx_size,
        )?;
        idx_offset += idx_size;
      }
    }

    // Will project scissor/clipping rectangles into framebuffer space
    let (clip_off, clip_scale, fb_size) = unsafe {
      let io = ImGui_GetIO();
      (
        (*draw_data).DisplayPos,        // (0,0) unless using multi-viewports
        (*draw_data).FramebufferScale,  // (1,1) unless using retina display which are often (2,2)
        (*io).DisplaySize,
      )
    };

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
        fb_size.x,
        fb_size.y,
        0.0,
        1.0,
      )],
    );

    // Setup scale and translation:
    // Our visible imgui space lies from draw_data->DisplayPps (top left) to draw_data->DisplayPos+data_data->DisplaySize (bottom right). DisplayPos is (0,0) for single viewport apps.
    let scale = [
      2.0 / fb_size.x,
      2.0 / fb_size.y,
    ];
    let translate = [
      -1.0 - clip_off.x * scale[0],
      -1.0 - clip_off.y * scale[1],
    ];
    command_buffers.push_constants_f32(
      index,
      &self.pipeline,
      hala_gfx::HalaShaderStageFlags::VERTEX,
      0,
      &scale,
    );
    command_buffers.push_constants_f32(
      index,
      &self.pipeline,
      hala_gfx::HalaShaderStageFlags::VERTEX,
      std::mem::size_of_val(&scale) as u32,
      &translate,
    );

    // Render command list.
    unsafe {
      let mut vtx_offset = 0;
      let mut idx_offset = 0;
      for n in 0..(*draw_data).CmdListsCount {
        let cmd_list = (*draw_data).CmdLists[n as usize];
        for cmd_i in 0..(*cmd_list).CmdBuffer.Size {
          let cmd = &(*cmd_list).CmdBuffer[cmd_i as usize];

          match cmd.UserCallback {
            Some(cb) => {
              cb(cmd_list, cmd);
            },
            None => {
              // Project scissor/clipping rectangles into framebuffer space.
              let mut clip_min = ImVec2 {
                x: (cmd.ClipRect.x - clip_off.x) * clip_scale.x,
                y: (cmd.ClipRect.y - clip_off.y) * clip_scale.y,
              };
              let mut clip_max = ImVec2 {
                x: (cmd.ClipRect.z - clip_off.x) * clip_scale.x,
                y: (cmd.ClipRect.w - clip_off.y) * clip_scale.y,
              };

              // Clamp to viewport as vkCmdSetScissor() won't accept values that are off bounds.
              if clip_min.x < 0.0 { clip_min.x = 0.0; }
              if clip_min.y < 0.0 { clip_min.y = 0.0; }
              if clip_max.x > fb_size.x { clip_max.x = fb_size.x; }
              if clip_max.y > fb_size.y { clip_max.y = fb_size.y; }
              if clip_max.x <= clip_min.x || clip_max.y <= clip_min.y {
                continue;
              }

              // Apply scissor/clipping rectangle.
              command_buffers.set_scissor(
                index,
                0,
                &[
                  (clip_min.x as i32, clip_min.y as i32, (clip_max.x - clip_min.x) as u32, (clip_max.y - clip_min.y) as u32)
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
                cmd.ElemCount,
                1,
                cmd.IdxOffset + idx_offset,
                cmd.VtxOffset as i32 + vtx_offset,
                0,
              );
            }
          }
        }

        idx_offset += (*cmd_list).IdxBuffer.Size as u32;
        vtx_offset += (*cmd_list).VtxBuffer.Size;
      }
    }

    // Note: at this point both vkCmdSetViewport() and vkCmdSetScissor() have been called.
    // Our last values will leak into user/application rendering.
    command_buffers.set_scissor(
      index,
      0,
      &[
        (0, 0, fb_size.x as u32, fb_size.y as u32)
      ],
    );

    core::result::Result::Ok(())
  }

  /// Whether any mouse button is down.
  /// return: Whether any mouse button is down.
  pub fn is_any_mouse_down(&self) -> bool {
    unsafe {
      ImGui_IsAnyMouseDown()
    }
  }

  /// Get the display scale.
  /// return: The display scale.
  pub fn get_display_framebuffer_scale(&self) -> (f32, f32) {
    unsafe {
      let io = ImGui_GetIO();
      ((*io).DisplayFramebufferScale.x, (*io).DisplayFramebufferScale.y)
    }
  }

  /// Get font size.
  /// return: The font size.
  pub fn get_font_size(&self) -> f32 {
    unsafe {
      ImGui_GetFontSize()
    }
  }

  /// Add a key event.
  /// param key: The key.
  /// param is_down: Whether the key is down.
  pub fn add_key_event(&self, key: ImGuiKey, is_down: bool) {
    unsafe {
      let io = ImGui_GetIO();
      ImGuiIO_AddKeyEvent(io, key, is_down)
    }
  }

  /// Add a input character.
  /// param char: The character.
  pub fn add_input_character(&self, char: u32) {
    unsafe {
      let io = ImGui_GetIO();
      ImGuiIO_AddInputCharacter(io, char)
    }
  }

  /// Add a focus event.
  /// param is_focused: Whether the window is focused.
  pub fn add_focus_event(&self, is_focused: bool) {
    unsafe {
      let io = ImGui_GetIO();
      ImGuiIO_AddFocusEvent(io, is_focused)
    }
  }

  /// Add a mouse position event.
  /// param x: The x position.
  /// param y: The y position.
  pub fn add_mouse_pos_event(&self, x: f32, y: f32) {
    unsafe {
      let io = ImGui_GetIO();
      ImGuiIO_AddMousePosEvent(io, x, y)
    }
  }

  /// Add a mouse button event.
  /// param button: The button.
  /// param is_down: Whether the button is down.
  pub fn add_mouse_button_event(&self, button: ImGuiMouseButton, is_down: bool) {
    unsafe {
      let io = ImGui_GetIO();
      ImGuiIO_AddMouseButtonEvent(io, button, is_down)
    }
  }

  /// Add a mouse wheel event.
  /// param h: The horizontal delta value.
  /// param v: The vertical delta value.
  pub fn add_mouse_wheel_event(&self, h: f32, v: f32) {
    unsafe {
      let io = ImGui_GetIO();
      ImGuiIO_AddMouseWheelEvent(io, h, v)
    }
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