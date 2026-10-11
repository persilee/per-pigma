# pigma 定制版 — 修改记录与编译指南

> 基于 pigma v0.2.15（GitHub: akirco/pigma，main 分支），在 macOS（Apple Silicon）上编译。
> 源码位置：`./pigma-src-modified/`（含全部修改，可直接编译）
> 备份压缩包：`./pigma-src-modified.tar.gz`

---

## 一、修改总览（共 5 处源码 + 1 处配置）

### 1. 歌词颜色调亮（`src/ui/lyrics.rs`）

**问题**：非当前行歌词是深灰色（136/85），在深色背景下看不清；翻译歌词更暗（180/100/60）。

**修改**：

| 位置 | 原颜色 | 新颜色 |
|---|---|---|
| 主歌词 · 近行（距当前行 ≤2 行） | `Rgb(136,136,136)` | `Rgb(255,251,235)` = `#fffbeb`（暖白） |
| 主歌词 · 远行（更远） | `Rgb(85,85,85)` | `Rgb(248,248,242)` = `#f8f8f2`（亮白） |
| 翻译 · 当前行（斜体） | `Rgb(180,180,180)` | `Rgb(210,210,210)` |
| 翻译 · 近行 | `Rgb(100,100,100)` | `Rgb(190,190,190)` |
| 翻译 · 远行 | `Rgb(60,60,60)` | `Rgb(160,160,160)` |

> 当前播放行的渐变高亮、正在唱到的「白字 + accent 底」未改动。

### 2. 列表标题：去掉左右箭头 + 前面加图标（`src/ui.rs`）

**问题**：所有内容列表标题默认是 `► 名称 (count/total) ◄`，左右箭头不好看。

**修改**：默认标题模板改为 `\u{ec1b} 名称 (count/total)`（U+EC1B 是你指定的 Nerd Font 图标，位于标题前）。

```rust
// 修改前
"\u{25BA} {name} ({count}/{total}) \u{25C4}"
// 修改后
"\u{ec1b} {name} ({count}/{total})"
```

> 注意：`\u{ec1b}` 需要你的终端字体（Nerd Font）包含该码位，否则显示为方块/空白。想换图标改这里即可。

### 3. 导航选中项：菱形 → 矩形（`src/ui/navigation.rs`）

**问题**：导航选中项用 Powerline 箭头 `\u{E0B2}` + `\u{E0B0}` 包裹，呈现两端尖角的菱形/胶囊形。

**修改**（`draw` 左侧导航 和 `draw_top` 顶部导航 两处）：
- 删除两侧箭头字符，选中项变为纯 accent 背景的**矩形**高亮块
- 选中文字颜色从 `colors.surface`（你的透明主题里是 `reset` → 白色）改为 **`#282a36`**（`Rgb(40,42,54)`）

### 4. 歌曲列表选中行：文字色改深（`src/ui/table.rs`）

**问题**：选中行文字用 `colors.bg`（你的透明主题里是 `reset` → 白色），在 accent 高亮底上对比不清。

**修改**：Row 模式和 Cell 模式的选中样式 `.fg(colors.bg)` → `.fg(Color::Rgb(40, 42, 54))`（`#282a36`）。

### 5. 播放队列高亮：文字色改深（`src/ui/queue.rs`）

**修改**（3 处）：
- `tab_style` 高亮 tab：`.fg(colors.bg)` → `.fg(Color::Rgb(40,42,54))`
- 正在播放行：`.fg(colors.surface)` → `.fg(Color::Rgb(40,42,54))`
- 选中行：`.fg(colors.bg)` → `.fg(Color::Rgb(40,42,54))`

### 6. 配置文件（`~/Library/Application Support/pigma/config.toml`）

- `default_theme`：`"dracula"` → `"dracula-transparent"`
- 新增自定义主题 `dracula-transparent`：`bg = "reset"`、`surface = "reset"`（透明背景，继承终端背景），其余颜色沿用 dracula 配色

> 原配置备份在 `config.toml.bak`。

---

## 二、编译方法（以后自己改完源码怎么编译）

### 环境（已装好）

```bash
# Rust 工具链（rustup 方式，已安装）
export PATH="$HOME/.cargo/bin:$PATH"
cargo --version   # cargo 1.99.0
```

### 编译

```bash
cd pigma-src-modified
cargo build --release
# 首次约 20 分钟，之后增量编译约 1 分钟
# 产物：target/release/pigma
```

> 源码目录里已包含子模块 `crates/y7dl`（已拉取），无需额外操作。
> 如果换了新机器/重新 clone，需要先 `git submodule update --init --recursive`。

### 替换安装

```bash
# 1) 备份当前版
cp /opt/homebrew/bin/pigma /opt/homebrew/bin/pigma.brew.bak

# 2) 替换（若提示权限不足加 sudo）
cp target/release/pigma /opt/homebrew/bin/pigma

# 3) 验证
shasum /opt/homebrew/bin/pigma   # 与 target/release/pigma 一致即可
```

### 生效

完全退出 pigma 后重新 `pigma`（运行中的进程不会加载新二进制）。

---

## 三、注意事项

1. **brew 会覆盖你的修改**：`brew upgrade pigma` 会把 `/opt/homebrew/bin/pigma` 换回官方版。升级后按上面步骤重新编译替换即可。
2. **字体**：pigma 界面（图标、边框）依赖 Nerd Fonts 终端字体；`\u{ec1b}` 等图标码位需字体支持。
3. **透明背景**：主题 `bg/surface = "reset"` 继承终端背景，终端模拟器需开启窗口透明度才能透出桌面。
4. **改颜色**：所有高亮选中文字色集中在 `navigation.rs`、`table.rs`、`queue.rs` 的 `Color::Rgb(40, 42, 54)`，搜索 `Rgb(40, 42, 54)` 即可全局定位。
5. **改标题图标**：在 `src/ui.rs` 搜索 `\u{ec1b}`。
6. **改歌词颜色**：在 `src/ui/lyrics.rs` 搜索 `Rgb(255, 251, 235)` 和 `Rgb(248, 248, 242)`。
