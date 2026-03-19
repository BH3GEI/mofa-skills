---
name: pptx
description: "Presentation creation, editing, and analysis. When Claude needs to work with presentations (.pptx files) for: (1) Creating new presentations, (2) Modifying or editing content, (3) Working with layouts, (4) Adding comments or speaker notes, or any other presentation tasks"
license: Proprietary. LICENSE.txt has complete terms
---

# PPT演示文稿处理专家 Real Skill

## 一句话说明

从 HTML 自动生成精美 PPT，支持演示文稿编辑、批注管理和内容提取。

**Presentation Creation & Editing** - Creating presentations from HTML, editing content, managing comments, extracting text.

---

## 使用场景

**适用**：
- 从数据/HTML 自动生成标准化 PPT 报告
- 创建精美的演示文稿（销售、培训、汇报）
- 提取 PPT 文本、批注、演讲者备注
- 修改幻灯片内容、布局、主题
- 批量生成多个演示文稿

**不适用**：
- 复杂动画制作（用专业软件 / use professional software）
- 实时协同编辑（用 Google Slides / use Google Slides for real-time collaboration）

---

## 核心流程

```
用户需求 → 内容分析 → 配色选择 → 工具选择 → 生成 PPT → 验证输出
User request → Content analysis → Color palette → Tool selection → Generate PPT → Validate
```

**任务类型判断 (Task Classification)**：

### Creating Presentations (创建演示文稿)
- **HTML to PPT Workflow (推荐 / RECOMMENDED)**
  - Write HTML slides with styling
  - Convert using html2pptx library
  - Accurate positioning and formatting
  - Web-safe fonts only (Arial, Helvetica, Georgia, etc.)
  
- **Python-PPTX Workflow**
  - Programmatic slide creation
  - Batch generation from data
  - Template-based automation

### Reading & Analysis (读取与分析)
- **Text extraction** → markitdown (convert to markdown)
- **Comments & Notes** → Unpack OOXML and read XML
- **Design analysis** → Extract theme, colors, fonts from XML

### Editing Existing (编辑已有文稿)
- **Simple edits** → python-pptx library
- **Complex edits** → Unpack OOXML, modify XML, repack

---

## 任务完成标准

**必须满足**（缺一不可）：
- PPT 可正常打开并播放
- 幻灯片内容清晰可读
- 配色协调，视觉层次分明
- 布局合理，文字不拥挤

**质量评级**：
- ⭐⭐⭐⭐⭐ 优秀 - 设计精美 + 内容完整 + 自动化生成
- ⭐⭐⭐ 及格 - 内容正确 + PPT 可用 + 格式规范
- ⭐ 失败 - PPT 损坏或内容错误

---

## 参考资料（供 AI 使用）

| 类型 | 路径 | 说明 |
|-----|------|------|
| 核心文档 | `docs/00-SKILL-完整制作指南.md` | Complete PPT creation guide (~500 lines) |
| OOXML库 | `ooxml/` | Low-level XML operations |
| 工具脚本 | `scripts/unpack.py` | Unpack PPTX files |
| 工具脚本 | `scripts/pack.py` | Pack PPTX files |

---

## 关键原则（AI 必读 / Critical Principles）

### 1. Design First - Content-Informed Approach (设计先行)
**CRITICAL**: Before creating any presentation, analyze the content:
- **Consider subject matter**: What is this about? What tone/industry/mood?
- **Check branding**: If company mentioned, consider brand colors
- **Match palette to content**: Select colors reflecting the subject
- **State your approach**: Explain design choices BEFORE writing code

### 2. Web-Safe Fonts Only (仅使用 Web 安全字体)
**Allowed fonts**: Arial, Helvetica, Times New Roman, Georgia, Courier New, Verdana, Tahoma, Trebuchet MS, Impact
- **No custom fonts**: They won't render correctly
- **Font pairing**: Use max 2-3 fonts per presentation

### 3. Visual Hierarchy (视觉层次)
- **Extreme size contrast**: 72pt headlines vs 11pt body text
- **Weight variation**: Bold headers, normal body, light captions
- **Color emphasis**: Use accent color for key information
- **Clear structure**: Title → Main content → Supporting details

### 4. Chart & Table Layout Rules (图表布局规则)
**PREFERRED - Two-column layout**:
- Header spanning full width
- Text/bullets in one column (40%) + Chart/table in other (60%)
- Better balance and readability

**Alternative - Full-slide layout**:
- Chart/table takes entire slide for maximum impact

**NEVER vertically stack**:
- ❌ Do NOT place charts/tables below text in single column
- ❌ Causes poor readability and layout issues

### 5. Color Palette Selection (18种配色方案)
**Think beyond defaults**: What colors genuinely match this topic?
**Consider multiple angles**: Topic, industry, mood, energy, audience, brand
**Be adventurous**: Healthcare doesn't have to be green, finance doesn't have to be navy

---

## 18种精选配色方案 (Color Palettes)

### Business & Professional (商业/专业)
1. **Classic Blue**: Navy (#1C2833), Slate (#2E4053), Silver (#AAB7B8), Off-white (#F4F6F6)
2. **Black & Gold**: Gold (#BF9A4A), Black (#000000), Cream (#F4F6F6)
3. **Charcoal & Red**: Charcoal (#292929), Red (#E33737), Light Gray (#CCCBCB)

### Creative & Vibrant (创意/活力)
4. **Teal & Coral**: Teal (#5EA8A7), Deep Teal (#277884), Coral (#FE4447), White (#FFFFFF)
5. **Retro Rainbow**: Purple (#722880), Pink (#D72D51), Orange (#EB5C18), Amber (#F08800), Gold (#DEB600)
6. **Vibrant Orange**: Orange (#F96D00), Light Gray (#F2F2F2), Charcoal (#222831)
7. **Bold Red**: Red (#C0392B), Bright Red (#E74C3C), Orange (#F39C12), Yellow (#F1C40F), Green (#2ECC71)

### Natural & Warm (自然/温暖)
8. **Sage & Terracotta**: Sage (#87A96B), Terracotta (#E07A5F), Cream (#F4F1DE), Charcoal (#2C2C2C)
9. **Forest Green**: Black (#191A19), Green (#4E9F3D), Dark Green (#1E5128), White (#FFFFFF)
10. **Vintage Earthy**: Mustard (#E3B448), Sage (#CBD18F), Forest Green (#3A6B35), Cream (#F4F1DE)
11. **Cream & Forest**: Cream (#FFE1C7), Forest Green (#40695B), White (#FCFCFC)

### Elegant & Soft (优雅/柔和)
12. **Warm Blush**: Mauve (#A49393), Blush (#EED6D3), Rose (#E8B4B8), Cream (#FAF7F2)
13. **Coastal Rose**: Old Rose (#AD7670), Beaver (#B49886), Eggshell (#F3ECDC), Ash Gray (#BFD5BE)
14. **Pink & Purple**: Pink (#F8275B), Coral (#FF574A), Rose (#FF737D), Purple (#3D2F68)

### Luxury & Bold (奢华/大胆)
15. **Burgundy Luxury**: Burgundy (#5D1D2E), Crimson (#951233), Rust (#C15937), Gold (#997929)
16. **Deep Purple & Emerald**: Purple (#B165FB), Dark Blue (#181B24), Emerald (#40695B), White (#FFFFFF)

### Fresh & Modern (清新/现代)
17. **Lime & Plum**: Lime (#C5DE82), Plum (#7C3A5F), Coral (#FD8C6E), Blue-Gray (#98ACB5)
18. **Orange & Turquoise**: Light Orange (#FC993E), Grayish Turquoise (#667C6F), White (#FCFCFC)

---

## 快速命令参考 (Quick Commands)

### Text Extraction (文本提取)
```bash
# Convert PPTX to markdown
python -m markitdown presentation.pptx
```

### Create Simple Presentation (创建简单演示文稿)
```python
from pptx import Presentation

prs = Presentation()
slide = prs.slides.add_slide(prs.slide_layouts[0])  # Title slide
title = slide.shapes.title
subtitle = slide.placeholders[1]

title.text = "Hello World!"
subtitle.text = "My First Presentation"

prs.save("output.pptx")
```

### Unpack PPTX for Analysis (解压分析)
```bash
# Unpack PPTX to access raw XML
python ooxml/scripts/unpack.py presentation.pptx output_dir

# Key files to examine:
# ppt/presentation.xml - Main metadata
# ppt/slides/slide1.xml - Individual slide contents
# ppt/theme/theme1.xml - Theme colors and fonts
# ppt/comments/ - Comments
# ppt/notesSlides/ - Speaker notes
```

### Repack After Editing (编辑后打包)
```bash
# Pack modified XML back to PPTX
python ooxml/scripts/pack.py output_dir new_presentation.pptx
```

### HTML to PPT Workflow (HTML 转 PPT)
```python
# Step 1: Write HTML slides
html_content = """
<!DOCTYPE html>
<html>
<head>
    <style>
        .slide { width: 960px; height: 720px; padding: 40px; }
        h1 { font-family: Arial; font-size: 48px; color: #1C2833; }
        p { font-family: Arial; font-size: 24px; color: #2E4053; }
    </style>
</head>
<body>
    <div class="slide">
        <h1>Welcome</h1>
        <p>This is my presentation</p>
    </div>
</body>
</html>
"""

# Step 2: Convert to PPTX
from html2pptx import html2pptx
with open("slides.html", "w") as f:
    f.write(html_content)
html2pptx("slides.html", "output.pptx")
```

---

## 依赖安装 (Dependencies Installation)

### Python Dependencies
```bash
pip install -r requirements.txt
# Includes: python-pptx, html2pptx, markitdown
```

---

## 常见场景示例 (Common Scenarios)

### Scenario 1: Create Professional Title Slide (创建专业标题页)
```python
from pptx import Presentation
from pptx.util import Inches, Pt
from pptx.dml.color import RGBColor

prs = Presentation()
slide = prs.slides.add_slide(prs.slide_layouts[6])  # Blank layout

# Add title
left = Inches(1)
top = Inches(2.5)
width = Inches(8)
height = Inches(1.5)
title_box = slide.shapes.add_textbox(left, top, width, height)
title_frame = title_box.text_frame
title_frame.text = "2024 Sales Report"

# Format title
title_para = title_frame.paragraphs[0]
title_para.font.size = Pt(72)
title_para.font.bold = True
title_para.font.color.rgb = RGBColor(28, 40, 51)  # Navy

# Add subtitle
subtitle_box = slide.shapes.add_textbox(left, Inches(4), width, Inches(0.5))
subtitle_frame = subtitle_box.text_frame
subtitle_frame.text = "Q4 Performance Review"
subtitle_para = subtitle_frame.paragraphs[0]
subtitle_para.font.size = Pt(24)
subtitle_para.font.color.rgb = RGBColor(46, 64, 83)  # Slate

prs.save("title_slide.pptx")
```

### Scenario 2: Extract All Speaker Notes (提取所有演讲者备注)
```bash
# Step 1: Unpack PPTX
python ooxml/scripts/unpack.py presentation.pptx unpacked/

# Step 2: Extract notes using grep
cd unpacked/ppt/notesSlides/
grep -r "a:t>" . | sed 's/<[^>]*>//g' > ../../all_notes.txt
```

### Scenario 3: Batch Generate Slides from Data (批量生成幻灯片)
```python
from pptx import Presentation
from pptx.util import Inches, Pt

data = [
    {"name": "Product A", "sales": 1200, "growth": "15%"},
    {"name": "Product B", "sales": 980, "growth": "8%"},
    {"name": "Product C", "sales": 1450, "growth": "22%"}
]

prs = Presentation()

for item in data:
    slide = prs.slides.add_slide(prs.slide_layouts[1])  # Title and content
    title = slide.shapes.title
    title.text = item["name"]
    
    body = slide.placeholders[1]
    tf = body.text_frame
    tf.text = f"Sales: ${item['sales']}M"
    
    p = tf.add_paragraph()
    p.text = f"Growth: {item['growth']}"
    p.level = 1

prs.save("batch_report.pptx")
```
