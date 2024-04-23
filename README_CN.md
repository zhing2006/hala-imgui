# hala-imgui
[![License](https://img.shields.io/badge/License-GPL3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![MSRV](https://img.shields.io/badge/rustc-1.70.0+-ab6000.svg)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)

[English](README.md) | [中文](README_CN.md) | [日本語](README_JP.md) | [한국어](README_KO.md)

## 简介
`hala-imgui`是一个基于`hala-gfx`开发的，使用IMGUI进行UI渲染的应用程序框架。IMGUI绑定部分使用了`easy-imgui-sys`，平台实现部分使用了`winit`。

## 功能特点
- **目前正在开发阶段，暂时不可用**

## 安装
要在你的Rust项目中使用`hala-imgui`，同级目录下必须现有`hala-gfx`，`hala-imgui`编译时会去"../hala-gfx"搜索。
你可以通过在`Cargo.toml`文件中添加以下依赖来使用`hala-imgui`：

```toml
[dependencies]
hala-renderer = { path = "./hala-imgui" }
```

确保你的系统已经安装了Rust编程环境和cargo包管理器。

## 依赖关系
`hala-imgui`依赖于[hala-gfx](https://github.com/zhing2006/hala-gfx)。

请确保`hala-gfx`依赖项在使用`hala-imgui`之前已正确放到同级目录。

## 贡献
欢迎任何形式的贡献，无论是bug报告或是代码贡献。

## 许可证
hala-renderer根据《[GNU General Public License v3.0许可证](LICENSE)》开源。

## 联系方式
如果你有任何问题或建议，请通过创建一个issue来联系。