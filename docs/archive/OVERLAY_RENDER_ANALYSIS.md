# OVERLAY RENDER ANALYSIS

## 1. Overlay 背景绘制代码

```rust
// 第 146 行
let (win_w, win_h) = canvas.output_size().unwrap_or((960, 720));

// 第 149-151 行
canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
canvas.set_draw_color(Color::RGBA(0, 0, 0, 200));
let _ = canvas.fill_rect(Rect::new(0, 0, win_w, win_h));
```

| 属性 | 值 |
|------|-----|
| 坐标 | (0, 0) — 窗口左上角 |
| 宽 | `win_w` — 来自 `canvas.output_size()`，预期 960 |
| 高 | `win_h` — 来自 `canvas.output_size()`，预期 720 |
| RGBA | (0, 0, 0, 200) — 黑色，alpha 200/255 ≈ 78% 不透明 |
| 混合模式 | `BlendMode::Blend` — alpha 混合 |

注意：`fill_rect` 返回值被 `let _ =` 丢弃，错误（如有）被静默忽略。

---

## 2. 文字绘制代码

### 字体数据来源

```rust
// 第 43-117 行
fn char_bitmap(c: char) -> [u8; 7]
```

- 内置 5×7 位图字体，硬编码在代码中
- 支持：A-Z、a-z、0-2、`-`、`/`、`(`、`)`、空格（默认全零）
- 每个字符 7 行，每行 5 位（存储在 `u8` 的低 5 位）

### 绘制位置

```rust
// 标题 — 第 160-161 行
let title = "TurtleBox - Controls";
let tw = title.len() as i32 * 6 * scale;  // 19 chars × 18 = 342px
draw_text(canvas, title, (win_w as i32 - tw) / 2, y, scale, Color::RGB(255, 255, 0));
// 位置: ((960 - 342) / 2, 30) = (309, 30)  黄色
```

```rust
// 列标题 — 第 169-170 行
draw_text(canvas, "Keyboard (P2)", col1, y, scale, Color::RGB(100, 200, 255));
// 位置: (50, 74)  浅蓝色
draw_text(canvas, "Xbox Controller (P1)", col2, y, scale, Color::RGB(100, 200, 255));
// 位置: (490, 74)  浅蓝色
```

```rust
// 内容行 — 第 191-198 行
// 左列起始: (50, 98), 行距 28px
// 右列起始: (490, 98), 行距 28px
// 颜色: RGB(180, 180, 180) 灰色
```

```rust
// 页脚 — 第 203-205 行
let footer = "Press F1 to close";
let fw = footer.len() as i32 * 6 * scale;  // 17 × 18 = 306px
draw_text(canvas, footer, (win_w as i32 - fw) / 2, y, scale, Color::RGB(255, 255, 255));
// 位置: ((960 - 306) / 2, ~250)  白色
```

### 绘制机制

```rust
// 第 119-134 行
fn draw_char(canvas, c, x, y, scale, color) {
    canvas.set_draw_color(color);
    for row in 0..7 {
        for col in 0..5 {
            if bit_set {
                let _ = canvas.fill_rect(Rect::new(x + col*scale, y + row*scale, scale, scale));
            }
        }
    }
}
```

- 每个像素用 `fill_rect` 绘制一个 `scale × scale`（3×3）的矩形
- `fill_rect` 返回值被 `let _ =` 丢弃

---

## 3. 渲染顺序

```rust
// 第 518-527 行
canvas.clear();                    // 1. 清空画布
canvas.copy(&texture, None, None)? // 2. 绘制游戏画面（NES 输出 256×240 缩放到窗口）

if show_help {
    render_help_overlay(&mut canvas); // 3. 绘制 overlay（在游戏画面之上）
}

canvas.present();                  // 4. 提交到屏幕
```

**渲染顺序确认：游戏画面 → Overlay → canvas.present()**

顺序正确。Overlay 绘制在游戏画面之上，然后一起提交。

---

## 4. "render_help_overlay() 被调用，但屏幕没有变化" 的可能原因

### 原因 1：`output_size()` 返回值异常

```rust
let (win_w, win_h) = canvas.output_size().unwrap_or((960, 720));
```

`canvas.output_size()` 在某些 SDL2 版本或 macOS 视网膜显示器上可能返回逻辑像素而非物理像素。如果返回 `(0, 0)`，则 `fill_rect(Rect::new(0, 0, 0, 0))` 绘制零面积矩形，不可见。

**但** `unwrap_or((960, 720))` 会兜底到 960×720，除非 `output_size()` 返回 `Ok((0, 0))` — 这不会触发 `unwrap_or`。

### 原因 2：`set_blend_mode` 对后续 `fill_rect` 不生效

SDL2 中 `set_blend_mode` 设置的是**纹理**的混合模式，而非**绘制操作**的混合模式。`canvas.set_blend_mode()` 设置的是 canvas 的默认纹理混合模式。对于 `fill_rect` 等绘制操作，混合行为取决于渲染器实现：

- 在某些 SDL2 后端，`fill_rect` 始终使用源覆盖（忽略 alpha）
- 在其他后端，`fill_rect` 尊重 `set_blend_mode`

如果渲染器不支持 `fill_rect` 的 alpha 混合，背景将变为完全不透明的黑色矩形 — **应该可见**（全黑遮罩）。

### 原因 3：`canvas.copy()` 使用硬件纹理覆盖

`canvas.copy(&texture, ...)` 将 NES 输出复制到画布。如果 SDL2 使用硬件加速渲染，`copy` 操作可能写入独立的图层或缓冲区。后续的 `fill_rect` 和 `draw_text` 虽然在同一个 canvas 上执行，但渲染器可能在 `present()` 时以不同方式合并这些操作。

**可能性低**，因为 `clear()` → `copy()` → `fill_rect()` → `present()` 是标准的 SDL2 渲染管线。

### 原因 4：`let _ =` 静默吞掉错误

```rust
let _ = canvas.fill_rect(Rect::new(0, 0, win_w, win_h));
let _ = canvas.fill_rect(Rect::new(x, y, w, h));  // draw_char 内
```

所有 `fill_rect` 调用的返回值都被丢弃。如果 SDL2 渲染器处于错误状态（如渲染目标丢失），所有绘制调用返回 `Err` 但被静默忽略。

**这是最可疑的原因**。调试输出证明函数被调用了，但无法确认 `fill_rect` 是否成功执行。

### 原因 5：窗口焦点 / 最小化

如果窗口在后台或最小化，SDL2 可能跳过渲染或将内容输出到不可见的缓冲区。`present()` 成功返回但屏幕无变化。

**可能性低**，因为游戏画面本身是可见的（用户能操作角色）。

### 原因 6：视网膜显示器 DPI 缩放

macOS 视网膜显示器上，逻辑像素 ≠ 物理像素。`canvas.output_size()` 返回物理像素尺寸，但 `fill_rect` 使用逻辑像素。如果两者不一致，绘制内容可能出现在窗口的错误区域或被裁剪。

**可能性低**，但值得在运行时打印 `output_size()` 实际返回值验证。

---

## 总结：最可能的原因排序

| 优先级 | 原因 | 验证方法 |
|--------|------|----------|
| ⭐1 | `fill_rect` 返回 Err 但被 `let _ =` 吞掉 | 打印 `fill_rect` 返回值 |
| 2 | `output_size()` 返回意外值 | 打印 `(win_w, win_h)` |
| 3 | `set_blend_mode` 对 `fill_rect` 行为不一致 | 用纯色 (alpha=255) 测试 |
| 4 | 视网膜 DPI 缩放 | 打印物理/逻辑像素比 |
| 5 | 渲染器状态异常 | 检查 `canvas.renderer_info()` |

---

*Generated: 2026-06-03*
*Analysis: Static only — no code modifications, no compilation*
