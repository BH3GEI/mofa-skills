---
name: mofa-paddleocr
description: "Complex document parsing with PaddleOCR. Convert PDFs and document images to structured Markdown and JSON. Triggers: paddleocr, pdf parsing, document ocr, pdf to markdown, document extraction"
requires_bins: python3, pip
always: false
---

# MOFA PaddleOCR Document Parsing

基于 [PaddleOCR](https://github.com/PaddlePaddle/PaddleOCR) 的复杂文档解析工具，将 PDF 和文档图像智能转换为保留原始结构的 Markdown 和 JSON。

## Onboarding / 开始使用

### 前置要求

1. **Python 3.8+**
   ```bash
   python3 --version
   ```

2. **安装 PaddleOCR**
   ```bash
   # 安装 paddlepaddle (CPU 版本)
   pip install paddlepaddle

   # 或安装 paddlepaddle-gpu (GPU 版本，需要 CUDA)
   pip install paddlepaddle-gpu

   # 安装 paddleocr
   pip install paddleocr
   ```

3. **安装额外依赖**
   ```bash
   pip install pdf2image pymupdf Pillow
   ```

4. **验证安装**
   ```bash
   python3 -c "from paddleocr import PaddleOCR; print('PaddleOCR installed')"
   ```

### 快速开始

```bash
# 单张图片 OCR
python3 -c "
from paddleocr import PaddleOCR
ocr = PaddleOCR(use_angle_cls=True, lang='ch')
result = ocr.ocr('document.png', cls=True)
for line in result[0]:
    print(line[1][0])  # 提取文本
"

# PDF 解析
python3 -c "
from paddleocr import PaddleOCR
import fitz  # PyMuPDF

ocr = PaddleOCR(use_angle_cls=True, lang='ch')
pdf_path = 'document.pdf'
doc = fitz.open(pdf_path)

for page_num in range(len(doc)):
    page = doc[page_num]
    pix = page.get_pixmap(matrix=fitz.Matrix(2, 2))
    img_path = f'/tmp/page_{page_num}.png'
    pix.save(img_path)

    result = ocr.ocr(img_path, cls=True)
    print(f'=== Page {page_num + 1} ===')
    for line in result[0]:
        print(line[1][0])
"
```

### 故障排除

| 问题 | 解决方案 |
|-----|---------|
| `paddlepaddle not found` | 运行 `pip install paddlepaddle` |
| `CUDA error` | 使用 CPU 版本: `pip install paddlepaddle` |
| `pdf2image error` | 安装 poppler: `brew install poppler` (mac) / `apt-get install poppler-utils` (linux) |
| 中文识别效果差 | 确认 `lang='ch'` 参数 |
| 内存不足 | 降低 PDF 分辨率: `matrix=fitz.Matrix(1, 1)` |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      MOFA PADDLEOCR PIPELINE                                │
└─────────────────────────────────────────────────────────────────────────────┘

User Request (PDF/Image)
    ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 1: INPUT                                          │
│ ─────────────                                           │
│ • Validate file format (PDF/PNG/JPG)                    │
│ • Check file size                                       │
│ • Determine output format (markdown/json)               │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 2: CONVERT (PDF → Images)                         │
│ ───────────────────────────────                         │
│                                                         │
│   PDF input:                                            │
│   ┌─────────────────────────────────────────────┐       │
│   │  PyMuPDF (fitz)                             │       │
│   │  • Render pages to images                   │       │
│   │  • Multiplier: 2x for better OCR quality    │       │
│   │  • Output: page_1.png, page_2.png...        │       │
│   └─────────────────────────────────────────────┘       │
│                                                         │
│   Image input: Skip this phase                          │
│                                                         │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 3: OCR (PaddleOCR)                                │
│ ────────────────────────                                │
│                                                         │
│   PaddleOCR Pipeline:                                   │
│   ┌─────────────────────────────────────────────┐       │
│   │  1. Text Detection (DBNet)                  │       │
│   │     → Find text regions                     │       │
│   │                                             │       │
│   │  2. Text Recognition (CRNN)                 │       │
│   │     → Recognize characters                  │       │
│   │                                             │       │
│   │  3. Layout Analysis (optional)              │       │
│   │     → Detect tables, titles, paragraphs     │       │
│   │                                             │       │
│   │  4. Output: Bounding boxes + Text           │       │
│   └─────────────────────────────────────────────┘       │
│                                                         │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 4: STRUCTURE                                      │
│ ─────────────────                                       │
│ • Analyze text layout                                   │
│ • Detect tables and lists                               │
│ • Identify headings and paragraphs                      │
│ • Preserve reading order                                │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 5: OUTPUT                                         │
│ ─────────────                                           │
│                                                         │
│   Markdown format:                                      │
│   ┌─────────────────────────────────────────────┐       │
│   │  # Page 1                                   │       │
│   │                                             │       │
│   │  ## Title Section                           │       │
│   │  Extracted text content...                  │       │
│   │                                             │       │
│   │  | Table | Header |                         │       │
│   │  |----------|----------|                    │       │
│   │  | Cell 1  | Cell 2   |                     │       │
│   │                                             │       │
│   │  ---                                        │       │
│   │  # Page 2                                   │       │
│   └─────────────────────────────────────────────┘       │
│                                                         │
│   JSON format:                                          │
│   ┌─────────────────────────────────────────────┐       │
│   │  {                                          │       │
│   │    "pages": [{                              │       │
│   │      "page_num": 1,                         │       │
│   │      "text": "...",                         │       │
│   │      "blocks": [{                           │       │
│   │        "text": "...",                       │       │
│   │        "box": [x1,y1,x2,y2],                │       │
│   │        "confidence": 0.95                   │       │
│   │      }]                                     │       │
│   │    }]                                       │       │
│   │  }                                          │       │
│   └─────────────────────────────────────────────┘       │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Features

### 1. Multi-Format Input

| Format | Support | Notes |
|--------|---------|-------|
| PDF | ✅ | Renders each page to image |
| PNG | ✅ | Best quality |
| JPG/JPEG | ✅ | Compressed, may affect OCR |
| TIFF | ✅ | Multi-page supported |
| BMP | ✅ | Uncommon |

### 2. OCR Languages

```python
# 中文
ocr = PaddleOCR(use_angle_cls=True, lang='ch')

# 英文
ocr = PaddleOCR(use_angle_cls=True, lang='en')

# 多语言
ocr = PaddleOCR(use_angle_cls=True, lang='latin')  # 拉丁语系
```

### 3. Output Formats

```python
# 纯文本
for line in result[0]:
    print(line[1][0])

# 带置信度
for line in result[0]:
    text = line[1][0]
    confidence = line[1][1]
    box = line[0]
    print(f"{text} (conf: {confidence:.2f})")

# 结构化数据
{
  "text": "识别文本",
  "confidence": 0.95,
  "box": [[x1, y1], [x2, y2], [x3, y3], [x4, y4]]
}
```

## Usage Examples

### 示例 1: 基础图片 OCR

```python
from paddleocr import PaddleOCR

ocr = PaddleOCR(
    use_angle_cls=True,  # 方向分类
    lang='ch',           # 中文
    show_log=False       # 关闭冗余日志
)

result = ocr.ocr('document.png', cls=True)

# 提取所有文本
full_text = '\n'.join([line[1][0] for line in result[0]])
print(full_text)
```

### 示例 2: PDF 转 Markdown

```python
import fitz
from paddleocr import PaddleOCR
import os

def pdf_to_markdown(pdf_path, output_md):
    ocr = PaddleOCR(use_angle_cls=True, lang='ch', show_log=False)
    doc = fitz.open(pdf_path)

    markdown = []

    for page_num in range(len(doc)):
        # 渲染页面 (2x 分辨率提高 OCR 质量)
        page = doc[page_num]
        pix = page.get_pixmap(matrix=fitz.Matrix(2, 2))

        temp_img = f'/tmp/page_{page_num}.png'
        pix.save(temp_img)

        # OCR
        result = ocr.ocr(temp_img, cls=True)

        # 添加到 markdown
        markdown.append(f"# Page {page_num + 1}\n\n")

        if result[0]:
            for line in result[0]:
                text = line[1][0]
                confidence = line[1][1]
                if confidence > 0.8:  # 过滤低置信度
                    markdown.append(f"{text}\n")

        markdown.append("\n---\n\n")
        os.remove(temp_img)

    with open(output_md, 'w', encoding='utf-8') as f:
        f.writelines(markdown)

    print(f"Saved to {output_md}")

# 使用
pdf_to_markdown('input.pdf', 'output.md')
```

### 示例 3: 表格识别

```python
from paddleocr import PaddleOCR

# 使用表格识别模型
ocr = PaddleOCR(
    use_angle_cls=True,
    lang='ch',
    det_model_dir='ch_PP-OCRv4_det',
    rec_model_dir='ch_PP-OCRv4_rec',
    table=True,  # 启用表格识别
    show_log=False
)

result = ocr.ocr('table_image.png', cls=True)

# 表格结果在 result['table']
if 'table' in result:
    for table in result['table']:
        print(table['html'])  # HTML 格式表格
```

### 示例 4: 批量处理

```python
import glob
from paddleocr import PaddleOCR
import json

ocr = PaddleOCR(use_angle_cls=True, lang='ch', show_log=False)

def batch_process(image_dir, output_json):
    results = []

    for img_path in glob.glob(f"{image_dir}/*"):
        if img_path.lower().endswith(('.png', '.jpg', '.jpeg')):
            print(f"Processing {img_path}...")

            result = ocr.ocr(img_path, cls=True)

            # 提取文本和元数据
            texts = []
            if result[0]:
                for line in result[0]:
                    texts.append({
                        'text': line[1][0],
                        'confidence': float(line[1][1]),
                        'box': line[0]
                    })

            results.append({
                'file': img_path,
                'content': texts
            })

    with open(output_json, 'w', encoding='utf-8') as f:
        json.dump(results, f, ensure_ascii=False, indent=2)

# 使用
batch_process('./scanned_docs/', 'output.json')
```

## Configuration

### OCR 参数

```python
ocr = PaddleOCR(
    # 检测模型
    det_model_dir='path/to/det_model',
    det_limit_side_len=960,      # 检测边长限制
    det_limit_type='max',        # 限制类型

    # 识别模型
    rec_model_dir='path/to/rec_model',
    rec_image_shape='3, 48, 320',  # 输入形状
    rec_batch_num=6,              # 批次大小

    # 分类模型
    use_angle_cls=True,           # 启用方向分类
    cls_model_dir='path/to/cls_model',
    cls_batch_num=6,

    # 其他
    lang='ch',                    # 语言
    show_log=False,               # 显示日志
    use_gpu=False,                # 使用 GPU
    use_mp=False,                 # 多进程
    total_process_num=1           # 进程数
)
```

## Performance

| 任务 | 时间 (CPU) | 时间 (GPU) |
|-----|-----------|-----------|
| 单页 PDF (A4) | 2-5s | 0.5-1s |
| 单张图片 | 1-3s | 0.3-0.5s |
| 10页 PDF | 20-50s | 5-10s |

**优化建议:**
- 使用 GPU 加速
- 降低 PDF 渲染分辨率
- 启用多进程 `use_mp=True`
- 批量处理

## Integration with Other Skills

### 与 mofa-research-2.0 结合

```python
# 1. 使用 mofa-research-2.0 找到 PDF 文档
# 2. 使用 mofa-paddleocr 提取内容
# 3. 使用 mofa-research-2.0 分析提取的文本

research_result = research("topic")
pdf_url = research_result['pdf_url']

# 下载 PDF
# ...

# OCR 提取
text = paddleocr_extract('downloaded.pdf')

# 继续分析
analysis = analyze(text)
```

### 与 mofa-defuddle 结合

```python
# mofa-paddleocr: 处理扫描版 PDF (图片)
# mofa-defuddle: 处理文字版 PDF (提取)

if is_scanned_pdf(file):
    text = paddleocr_extract(file)
else:
    text = defuddle_extract(file)
```

## Output Requirements

### 必须生成的文件

```
./paddleocr/{task-slug}/
├── output.md              # Markdown 输出
├── output.json            # 结构化 JSON
├── metadata.json          # 处理元数据
│   ├── page_count
│   ├── processing_time
│   └── avg_confidence
└── images/                # 中间图片 (可选)
    ├── page_1.png
    └── ...
```

### 质量检查清单

- [ ] 所有页面已处理
- [ ] 文本提取完整
- [ ] 阅读顺序正确
- [ ] 置信度 > 0.8
- [ ] Markdown 格式正确
- [ ] 原始文件路径已记录

## Related Skills

### mofa-defuddle
Web 内容提取工具。

**区别:**
- mofa-defuddle: 提取网页/文档的文字内容
- mofa-paddleocr: OCR 识别图片中的文字

**组合使用:**
```
扫描版 PDF → mofa-paddleocr → 文字
文字版 PDF → mofa-defuddle → 文字
```

### mofa-research-2.0
深度研究管道。

**组合使用:**
```
mofa-research-2.0: 发现文档
mofa-paddleocr: 提取内容
mofa-research-2.0: 分析内容
```

### mofa-firecrawl / mofa-crawler
网页爬取工具。

**组合使用:**
```
mofa-firecrawl: 爬取网站获取 PDF
mofa-paddleocr: 提取 PDF 内容
```
