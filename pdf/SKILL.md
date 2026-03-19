---
name: pdf
description: Comprehensive PDF manipulation toolkit for extracting text and tables, creating new PDFs, merging/splitting documents, and handling forms. When Claude needs to fill in a PDF form or programmatically process, generate, or analyze PDF documents at scale.
license: Proprietary. LICENSE.txt has complete terms
---

# PDF文档处理专家 Real Skill

## 一句话说明

全面的 PDF 文档处理工具，支持文本表格提取、PDF 创建、文档操作和表单填写。

**PDF Processing Guide** - Comprehensive PDF manipulation toolkit for extracting text and tables, creating new PDFs, merging/splitting documents, and handling forms.

---

## 使用场景

**适用**：
- 从 PDF 提取文本、表格数据并导出 Excel
- 创建新的 PDF 文档（报告、证书、发票）
- 合并、拆分、旋转 PDF 文档
- 填写 PDF 表单（政府表单、申请表）
- 批量处理多个 PDF 文件

**不适用**：
- 扫描版 PDF（需先 OCR 处理 / requires OCR first）
- 复杂的 PDF 编辑（用专业软件 / use professional software）

---

## 核心流程

```
用户需求 → 判断任务类型 → 选择工具 → 执行操作 → 验证输出
User request → Task type classification → Tool selection → Execute → Validate
```

**任务类型判断 (Task Classification)**：

### Extracting Data (提取数据)
- **文本提取 (Text extraction)** → pdfplumber.extract_text()
  - Better accuracy than pypdf for text extraction
- **表格提取 (Table extraction)** → pdfplumber.extract_tables() + pandas
  - Intelligent table structure recognition
  - Export to Excel, CSV, JSON

### Creating PDFs (创建 PDF)
- **简单文档 (Simple documents)** → reportlab.pdfgen.canvas
  - Quick document creation with text and graphics
- **复杂布局 (Complex layouts)** → reportlab.platypus
  - Paragraphs, Tables, Images with advanced styling

### PDF Operations (PDF 操作)
- **合并 (Merge)** → pypdf.PdfWriter
  - Combine multiple PDFs into one
- **拆分 (Split)** → pypdf.PdfWriter (save each page separately)
- **旋转 (Rotate)** → page.rotate()
- **元信息 (Metadata)** → pypdf.PdfReader.metadata / PdfWriter.add_metadata()

### Form Processing (表单处理)
- **查看字段 (View fields)** → See forms.md
- **填写表单 (Fill forms)** → See forms.md
- **批量处理 (Batch processing)** → Loop + form filling API

---

## 任务完成标准

**必须满足**（缺一不可）：
- PDF 文件可正常打开
- 提取的数据完整准确
- 表格结构保留正确（行列对应）
- 创建的 PDF 内容清晰可读

**质量评级**：
- ⭐⭐⭐⭐⭐ 优秀 - 数据完整 + 格式规范 + 自动化批处理
- ⭐⭐⭐ 及格 - 数据正确 + PDF 可用
- ⭐ 失败 - 数据缺失或 PDF 损坏

---

## 参考资料（供 AI 使用）

| 类型 | 路径 | 说明 |
|-----|------|------|
| 核心文档 | `docs/00-SKILL-完整处理指南.md` | Complete PDF processing guide (~300 lines) |
| 表单处理 | `docs/forms.md` | PDF form filling detailed tutorial |
| 高级参考 | `docs/reference.md` | JavaScript libraries and advanced features |

---

## 关键原则（AI 必读 / Critical Principles）

### 1. Prefer pdfplumber for Extraction (优先使用 pdfplumber)
- **文本提取**: More accurate than pypdf
- **表格识别**: Strong table recognition capability
- **布局保留**: Preserves layout information
- **复杂 PDF**: Better for complex PDF processing

### 2. Table Data Normalization (表格数据规范化)
```python
# Best practice for table extraction
import pdfplumber
import pandas as pd

all_tables = []
with pdfplumber.open("document.pdf") as pdf:
    for page in pdf.pages:
        tables = page.extract_tables()
        for table in tables:
            if table:  # Check not empty
                df = pd.DataFrame(table[1:], columns=table[0])  # First row as header
                all_tables.append(df)

# Combine and clean
combined = pd.concat(all_tables, ignore_index=True)
combined.to_excel("output.xlsx", index=False)
```

### 3. Scanned PDFs Require OCR (扫描版需要 OCR)
- **识别**: If text extraction returns empty or garbled text
- **处理**: Must use OCR tools first (Tesseract, Adobe Acrobat, etc.)
- **告知用户**: Clearly inform user that OCR pre-processing is needed

### 4. Memory Management for Large Files (大文件内存管理)
- Batch processing for multiple PDFs
- Process pages sequentially rather than loading entire document
- Use generators when possible

---

## 快速命令参考 (Quick Commands)

### Text Extraction (文本提取)
```python
import pdfplumber

with pdfplumber.open("document.pdf") as pdf:
    for page in pdf.pages:
        text = page.extract_text()
        print(text)
```

### Table to Excel (表格转 Excel)
```python
import pdfplumber
import pandas as pd

all_tables = []
with pdfplumber.open("document.pdf") as pdf:
    for page in pdf.pages:
        tables = page.extract_tables()
        for table in tables:
            if table:
                df = pd.DataFrame(table[1:], columns=table[0])
                all_tables.append(df)

combined = pd.concat(all_tables, ignore_index=True)
combined.to_excel("extracted_tables.xlsx", index=False)
```

### Merge PDFs (合并 PDF)
```python
from pypdf import PdfWriter, PdfReader

writer = PdfWriter()
for pdf_file in ["doc1.pdf", "doc2.pdf", "doc3.pdf"]:
    reader = PdfReader(pdf_file)
    for page in reader.pages:
        writer.add_page(page)

with open("merged.pdf", "wb") as output:
    writer.write(output)
```

### Split PDF (拆分 PDF)
```python
from pypdf import PdfReader, PdfWriter

reader = PdfReader("large_document.pdf")
for i, page in enumerate(reader.pages):
    writer = PdfWriter()
    writer.add_page(page)
    with open(f"page_{i+1}.pdf", "wb") as output:
        writer.write(output)
```

### Create PDF (创建 PDF)
```python
from reportlab.lib.pagesizes import letter
from reportlab.pdfgen import canvas

c = canvas.Canvas("hello.pdf", pagesize=letter)
width, height = letter
c.drawString(100, height - 100, "Hello World!")
c.save()
```

### Extract Metadata (提取元信息)
```python
from pypdf import PdfReader

reader = PdfReader("document.pdf")
meta = reader.metadata
print(f"Title: {meta.title}")
print(f"Author: {meta.author}")
print(f"Pages: {len(reader.pages)}")
```

---

## 依赖安装 (Dependencies Installation)

### Python Dependencies
```bash
pip install -r requirements.txt
# Includes: pypdf, pdfplumber, reportlab, pandas, openpyxl
```

---

## 常见场景示例 (Common Scenarios)

### Scenario 1: Batch Extract Invoice Tables (批量提取发票表格)
```python
import pdfplumber
import pandas as pd
import glob

all_data = []
for pdf_file in glob.glob("invoices/*.pdf"):
    with pdfplumber.open(pdf_file) as pdf:
        for page in pdf.pages:
            tables = page.extract_tables()
            for table in tables:
                if table:
                    df = pd.DataFrame(table[1:], columns=table[0])
                    df['source_file'] = pdf_file
                    all_data.append(df)

combined = pd.concat(all_data, ignore_index=True)
combined.to_excel("all_invoices.xlsx", index=False)
```

### Scenario 2: Create Report with Table (生成带表格的报告)
```python
from reportlab.lib.pagesizes import letter
from reportlab.platypus import SimpleDocTemplate, Table, TableStyle, Paragraph
from reportlab.lib.styles import getSampleStyleSheet

doc = SimpleDocTemplate("report.pdf", pagesize=letter)
styles = getSampleStyleSheet()
story = []

# Title
story.append(Paragraph("Sales Report", styles['Title']))

# Table
data = [
    ['Product', 'Q1', 'Q2', 'Q3', 'Q4'],
    ['Product A', '1000', '1200', '1100', '1300'],
    ['Product B', '800', '900', '950', '1000']
]
t = Table(data)
t.setStyle(TableStyle([
    ('BACKGROUND', (0, 0), (-1, 0), 'grey'),
    ('TEXTCOLOR', (0, 0), (-1, 0), 'white'),
    ('ALIGN', (0, 0), (-1, -1), 'CENTER'),
    ('GRID', (0, 0), (-1, -1), 1, 'black')
]))
story.append(t)

doc.build(story)
```
