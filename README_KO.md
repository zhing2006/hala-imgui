# hala-imgui
[![License](https://img.shields.io/badge/License-GPL3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![MSRV](https://img.shields.io/badge/rustc-1.70.0+-ab6000.svg)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)

[English](README.md) | [中文](README_CN.md) | [日本語](README_JP.md) | [한국어](README_KO.md)

## 소개
`hala-imgui`는 `hala-gfx` 기반으로 개발된 애플리케이션 프레임워크로, IMGUI를 이용한 UI 렌더링을 제공합니다. IMGUI 바인딩 부분에는 `easy-imgui-sys`를 사용했으며, 플랫폼 구현부에는 `winit`이 사용되었습니다.

## 기능 특징
- **현재 개발 중으로, 아직 사용할 수 없습니다**

## 설치
Rust 프로젝트에서 `hala-imgui`를 사용하려면, 동일 디렉토리에 `hala-gfx`가 미리 있어야 하며, `hala-imgui`는 "../hala-gfx"에서 검색합니다.
`Cargo.toml` 파일에 다음 의존성을 추가하여 `hala-imgui`를 사용할 수 있습니다:

```toml
[dependencies]
hala-imgui = { path = "./hala-imgui" }
```

시스템에 Rust 프로그래밍 환경과 cargo 패키지 매니저가 설치되어 있는지 확인하십시오.

## 의존성
`hala-imgui`는 [hala-gfx](https://github.com/zhing2006/hala-gfx)에 의존합니다.

`hala-imgui`를 사용하기 전에 `hala-gfx` 의존성이 동일 디렉토리에 올바르게 배치되어 있는지 확인하십시오.

## 기여
버그 보고 또는 코드 기여 등 모든 종류의 기여를 환영합니다.

## 라이선스
`hala-imgui`는 GNU General Public License v3.0을 오픈 소스 라이선스로 사용합니다.

## 연락처
질문이나 제안이 있으시면 issue를 생성하여 연락주십시오.