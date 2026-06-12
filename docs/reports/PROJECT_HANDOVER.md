# PROJECT HANDOVER — TurtleBox

## 项目概述

TurtleBox 是一个 NES 模拟器，专为运行 **Teenage Mutant Ninja Turtles III: The Manhattan Project** 设计。

基于 Rust + SDL2 + tetanes-core 构建，可在 macOS 上运行。

---

## 1. 项目目录结构

```
忍者神龟3——FC模拟器/
├── Cargo.toml                    # Rust 项目配置
├── Cargo.lock                    # 依赖版本锁定
├── src/
│   └── main.rs                   # 主程序 (所有逻辑)
├── spritecans.nes                # 测试 ROM (公开领域)
├── TMNT3.nes                     # TMNT3 ROM (用户提供的合法 ROM)
├── target/                       # 编译输出
│   ├── debug/                    # Debug 构建
│   └── release/                  # Release 构建
├── .cargo/                       # Cargo 配置
├── .git/                         # Git 仓库
├── PROJECT_GUARDRAILS.md         # 项目规范
├── PROJECT_ROADMAP.md            # 项目路线图
├── CURRENT_PHASE.md              # 当前阶段状态
├── PROJECT_COMPLETION_REPORT.md  # 完成报告
└── PROJECT_HANDOVER.md           # 本交接文档
```

---

## 2. 核心源码说明

### main.rs 结构

```rust
// 1. 依赖导入
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::controller::{Button, Axis};
use tetanes_core::prelude::*;
use tetanes_core::input::JoypadBtnState;

// 2. 常量定义
const STICK_THRESHOLD: i16 = 16000;

// 3. 音频回调结构
struct NesAudioCallback { ... }

// 4. 主函数
fn main() -> Result<(), String> {
    // 4.1 命令行参数解析
    // 4.2 SDL2 初始化
    // 4.3 音频设备初始化
    // 4.4 tetanes-core 初始化
    // 4.5 ROM 加载
    // 4.6 手柄检测
    // 4.7 主循环 (事件处理 + 模拟 + 渲染)
}
```

### 代码行数

| 文件 | 行数 | 功能 |
|------|------|------|
| main.rs | ~300 | 所有逻辑 |
| Cargo.toml | ~10 | 依赖配置 |

---

## 3. SDL2 视频模块说明

### 初始化

```rust
let sdl_context = sdl2::init()?;
let video_subsystem = sdl_context.video()?;

let window = video_subsystem
    .window("TurtleBox", 960, 720)
    .position_centered()
    .build()?;

let mut canvas = window.into_canvas().build()?;
let texture_creator = canvas.texture_creator();

let mut texture = texture_creator
    .create_texture_streaming(PixelFormatEnum::RGBA32, 256, 240)?;
```

### 渲染流程

```
每帧循环:
1. deck.clock_frame()         // 模拟一帧
2. deck.frame_buffer()        // 获取 RGBA 像素 (256x240x4 = 245,760 bytes)
3. texture.update(pixels)     // 更新 SDL2 纹理
4. canvas.copy(texture)       // 复制到画布 (自动缩放到 960x720)
5. canvas.present()           // 显示到窗口
```

### 关键参数

| 参数 | 值 | 说明 |
|------|-----|------|
| 窗口大小 | 960x720 | 4x 缩放 |
| NES 分辨率 | 256x240 | 原始 NES 输出 |
| 像素格式 | RGBA32 | 4 bytes/pixel |
| 纹理类型 | Streaming | 每帧更新 |

---

## 4. SDL2 输入模块说明

### 键盘映射

```rust
// KeyDown 事件
Event::KeyDown { keycode: Some(key), .. } => {
    match key {
        Keycode::Up => deck.joypad_mut(Player::One).set_button(JoypadBtnState::UP, true),
        Keycode::Down => deck.joypad_mut(Player::One).set_button(JoypadBtnState::DOWN, true),
        Keycode::Left => deck.joypad_mut(Player::One).set_button(JoypadBtnState::LEFT, true),
        Keycode::Right => deck.joypad_mut(Player::One).set_button(JoypadBtnState::RIGHT, true),
        Keycode::Z => deck.joypad_mut(Player::One).set_button(JoypadBtnState::A, true),
        Keycode::X => deck.joypad_mut(Player::One).set_button(JoypadBtnState::B, true),
        Keycode::Return => deck.joypad_mut(Player::One).set_button(JoypadBtnState::START, true),
        Keycode::RShift => deck.joypad_mut(Player::One).set_button(JoypadBtnState::SELECT, true),
        _ => {}
    }
}
```

### Xbox 手柄映射

```rust
// 按钮事件
Event::ControllerButtonDown { button, .. } => {
    match button {
        // A/X/LB → NES A
        Button::A | Button::X | Button::LeftShoulder => {
            deck.joypad_mut(Player::One).set_button(JoypadBtnState::A, true);
        }
        // B/Y/RB → NES B
        Button::B | Button::Y | Button::RightShoulder => {
            deck.joypad_mut(Player::One).set_button(JoypadBtnState::B, true);
        }
        // DPad + Start/Back
        ...
    }
}

// 左摇杆事件
Event::ControllerAxisMotion { axis, value, .. } => {
    match axis {
        Axis::LeftX => {
            if value > STICK_THRESHOLD { /* RIGHT */ }
            else if value < -STICK_THRESHOLD { /* LEFT */ }
            else { /* 释放 */ }
        }
        Axis::LeftY => {
            if value > STICK_THRESHOLD { /* DOWN */ }
            else if value < -STICK_THRESHOLD { /* UP */ }
            else { /* 释放 */ }
        }
        _ => {}
    }
}
```

### 完整映射表

| 输入源 | NES 按钮 |
|--------|----------|
| 键盘 ↑↓←→ | Up/Down/Left/Right |
| 键盘 Z | A |
| 键盘 X | B |
| 键盘 Enter | Start |
| 键盘 Right Shift | Select |
| Xbox DPad | Up/Down/Left/Right |
| Xbox Left Stick | Up/Down/Left/Right |
| Xbox A/X/LB | A |
| Xbox B/Y/RB | B |
| Xbox Start | Start |
| Xbox Back | Select |

---

## 5. SDL2 音频模块说明

### 音频回调

```rust
struct NesAudioCallback {
    buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioCallback for NesAudioCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        let mut buffer = self.buffer.lock().unwrap();
        let len = out.len().min(buffer.len());

        // 复制样本到输出
        for i in 0..len {
            out[i] = buffer[i];
        }

        // 填充静音
        for i in len..out.len() {
            out[i] = 0.0;
        }

        // 移除已播放样本
        buffer.drain(..len);
    }
}
```

### 音频初始化

```rust
let audio_buffer: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));

let audio_spec = AudioSpecDesired {
    freq: Some(44100),
    channels: Some(1),
    samples: Some(1024),
};

let audio_device = audio_subsystem.open_playback(None, &audio_spec, |spec| {
    NesAudioCallback {
        buffer: audio_buffer.clone(),
    }
})?;

audio_device.resume();
```

### 音频管道

```
主循环:
1. deck.clock_frame()              // 模拟一帧 (产生音频样本)
2. deck.audio_samples()            // 获取 f32 样本 (~3607 samples/frame)
3. audio_buffer.lock().extend()    // 添加到共享缓冲区
4. deck.clear_audio_samples()      // 清除已读取样本

音频回调 (独立线程):
1. 从 audio_buffer 读取样本
2. 写入 SDL2 音频输出
3. 移除已播放样本
```

### 音频参数

| 参数 | 值 | 说明 |
|------|-----|------|
| Sample Rate | 44100 Hz | CD 音质 |
| Channels | 1 (Mono) | NES 原始单声道 |
| Buffer Size | 1024 samples | ~23ms 延迟 |
| Format | f32 | 32-bit 浮点 |

---

## 6. tetanes-core 集成方式

### 依赖配置

```toml
# Cargo.toml
[dependencies]
sdl2 = "0.38"
tetanes-core = "0.14"
```

### 核心 API

```rust
use tetanes_core::prelude::*;
use tetanes_core::input::JoypadBtnState;

// 创建模拟器
let mut deck = ControlDeck::with_config(Config {
    region: NesRegion::Ntsc,
    ..Default::default()
});

// 加载 ROM
deck.load_rom_path("game.nes")?;

// 模拟一帧
deck.clock_frame()?;

// 获取视频输出
let pixels: &[u8] = deck.frame_buffer();  // 256x240x4 RGBA

// 获取音频输出
let samples: &[f32] = deck.audio_samples();

// 设置输入
deck.joypad_mut(Player::One).set_button(JoypadBtnState::A, true);

// 清除音频缓冲
deck.clear_audio_samples();
```

### 数据流

```
ROM 文件
    ↓
ControlDeck.load_rom_path()
    ↓
ControlDeck.clock_frame()
    ↓
┌───────────────┬───────────────┐
↓               ↓               ↓
frame_buffer()  audio_samples() joypad_mut()
(RGBA 像素)     (f32 样本)      (输入状态)
↓               ↓               ↓
SDL2 Texture    SDL2 Audio      NES 模拟
```

---

## 7. TMNT3 兼容性说明

### ROM 规格

| 项目 | 规格 |
|------|------|
| 游戏名称 | Teenage Mutant Ninja Turtles III: The Manhattan Project |
| Mapper | 4 (MMC3) |
| PRG ROM | 256KB (16 x 16KB) |
| CHR ROM | 256KB (32 x 8KB) |
| Mirroring | Horizontal |
| Battery | No |
| Trainer | No |

### tetanes-core 支持

| 功能 | 支持状态 |
|------|----------|
| Mapper 4 (MMC3) | ✅ 完全支持 |
| MMC3 IRQ | ✅ 支持 |
| Bank Switching | ✅ 支持 |
| Horizontal Mirroring | ✅ 支持 |

### 兼容性验证

- ✅ ROM 加载成功
- ✅ 标题画面显示
- ✅ 游戏音乐播放
- ✅ 键盘输入响应
- ✅ Xbox 手柄输入响应
- ✅ 60 FPS 稳定运行

---

## 8. 未来开发建议

### 高优先级

| 功能 | 说明 | 难度 |
|------|------|------|
| 存档系统 | Save/Load State | 中 |
| 全屏模式 | F11 切换 | 低 |
| 音量控制 | +/- 调节音量 | 低 |
| 暂停功能 | P 键暂停 | 低 |

### 中优先级

| 功能 | 说明 | 难度 |
|------|------|------|
| 文件选择器 | 图形化 ROM 选择 | 中 |
| FPS 显示 | 帧率计数器 | 低 |
| 连发按钮 | Turbo A/B | 低 |
| 窗口缩放 | 可调整窗口大小 | 低 |

### 低优先级

| 功能 | 说明 | 难度 |
|------|------|------|
| 着色器 | CRT 滤镜效果 | 高 |
| 多手柄支持 | Player 2 | 中 |
| 调试工具 | PPU/CPU 查看器 | 高 |
| 网络对战 | 联机功能 | 很高 |

### 架构建议

如果需要扩展功能，建议重构为模块化结构：

```
src/
├── main.rs           # 入口点
├── app.rs            # 应用状态管理
├── video.rs          # SDL2 视频模块
├── audio.rs          # SDL2 音频模块
├── input.rs          # SDL2 输入模块
├── emulator.rs       # tetanes-core 封装
└── config.rs         # 配置管理
```

---

## 9. 已知风险

### 技术风险

| 风险 | 说明 | 缓解措施 |
|------|------|----------|
| tetanes-core 更新 | API 可能变化 | 锁定版本 0.14 |
| SDL2 兼容性 | macOS 版本差异 | 测试多版本 |
| 音频延迟 | 缓冲区大小影响 | 可调整 samples 参数 |
| 输入延迟 | 帧同步问题 | 已实现每帧同步 |

### 法律风险

| 风险 | 说明 | 缓解措施 |
|------|------|----------|
| ROM 版权 | TMNT3 ROM 受版权保护 | 仅使用合法获取的 ROM |
| 商标问题 | TMNT 商标 | 仅用于个人学习 |

### 性能风险

| 风险 | 说明 | 缓解措施 |
|------|------|----------|
| CPU 占用 | 模拟器计算密集 | Release 构建优化 |
| 内存泄漏 | 音频缓冲区增长 | 及时清理已播放样本 |

---

## 10. 新开发者如何快速上手

### 环境准备

```bash
# 1. 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 安装 SDL2
brew install sdl2

# 3. 克隆项目
git clone <repository-url>
cd 忍者神龟3——FC模拟器
```

### 构建运行

```bash
# Debug 构建
cargo build

# Release 构建 (推荐)
cargo build --release

# 运行 (使用测试 ROM)
cargo run --release

# 运行 TMNT3
cargo run --release -- /path/to/TMNT3.nes
```

### 理解代码

1. **阅读 main.rs** — 所有逻辑在一个文件，约 300 行
2. **理解数据流** — ROM → tetanes-core → SDL2
3. **查看 tetanes-core API** — `ControlDeck`, `JoypadBtnState`
4. **测试修改** — 修改后运行验证

### 关键文件

| 文件 | 重要性 | 说明 |
|------|--------|------|
| src/main.rs | ⭐⭐⭐ | 主程序，所有逻辑 |
| Cargo.toml | ⭐⭐ | 依赖配置 |
| tetanes-core 源码 | ⭐⭐ | 模拟器核心 (在 ~/.cargo/registry) |

### 调试技巧

```bash
# 查看编译错误
cargo build 2>&1

# 运行并查看输出
cargo run -- /path/to/ROM.nes 2>&1

# 检查依赖
cargo tree

# 清理重新构建
cargo clean && cargo build
```

### 常见问题

| 问题 | 解决方案 |
|------|----------|
| SDL2 找不到 | `brew install sdl2` |
| 编译错误 | 检查 Rust 版本 `rustc --version` |
| 无声音 | 检查系统音量和音频设备 |
| 手柄无响应 | 检查手柄连接和 SDL2 支持 |
| 画面卡顿 | 使用 Release 构建 `cargo build --release` |

---

## 附录: 依赖版本

| 依赖 | 版本 | 用途 |
|------|------|------|
| sdl2 | 0.38 | 图形/输入/音频 |
| tetanes-core | 0.14.1 | NES 模拟器核心 |
| rfd | 0.14 | 文件对话框 (当前未使用) |

---

## 附录: 项目规范摘要

来自 PROJECT_GUARDRAILS.md:

1. 一个阶段一个模块
2. 每个阶段必须可编译运行
3. 测试后再提交
4. 不要过早抽象
5. 不要泛化
6. 提交后原子化
7. 代码中不要有 TODO
8. 先读后写

---

**文档生成日期:** 2026-06-02
**项目状态:** 可运行
**维护者:** [待填写]
