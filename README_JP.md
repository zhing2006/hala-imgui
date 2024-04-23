# hala-imgui
[![License](https://img.shields.io/badge/License-GPL3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![MSRV](https://img.shields.io/badge/rustc-1.70.0+-ab6000.svg)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)

[English](README.md) | [中文](README_CN.md) | [日本語](README_JP.md) | [한국어](README_KO.md)

## 紹介
`hala-imgui`は`hala-gfx`に基づいて開発された、IMGUIを使用してUIレンダリングを行うアプリケーションフレームワークです。IMGUIバインディング部分は`easy-imgui-sys`を使用し、プラットフォーム実装部分には`winit`を使用しています。

## 機能特徴
- **現在開発中で、まだ使用できません**

## インストール
Rustプロジェクトで`hala-imgui`を使用するには、同じディレクトリに`hala-gfx`が存在している必要があります。`hala-imgui`は"../hala-gfx"で検索します。
`Cargo.toml`ファイルに以下の依存関係を追加することで`hala-imgui`を使用できます：

```toml
[dependencies]
hala-imgui = { path = "./hala-imgui" }
```

システムにRustプログラミング環境とcargoパッケージマネージャがインストールされていることを確認してください。

## 依存関係
`hala-imgui`は[hala-gfx](https://github.com/zhing2006/hala-gfx)に依存しています。

`hala-imgui`を使用する前に、`hala-gfx`依存関係が同じディレクトリに正しく配置されていることを確認してください。

## 貢献
バグ報告やコードの貢献など、あらゆる種類の貢献を歓迎します。

## ライセンス
`hala-imgui`は[GNU General Public License v3.0](LICENSE)でオープンソース化されています。

## 連絡先
ご質問や提案がある場合は、issueを作成してご連絡ください。