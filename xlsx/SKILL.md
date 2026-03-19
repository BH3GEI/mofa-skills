---
name: xlsx
description: "Comprehensive spreadsheet creation, editing, and analysis with support for formulas, formatting, data analysis, and visualization. When Claude needs to work with spreadsheets (.xlsx, .xlsm, .csv, .tsv, etc) for: (1) Creating new spreadsheets with formulas and formatting, (2) Reading or analyzing data, (3) Modify existing spreadsheets while preserving formulas, (4) Data analysis and visualization in spreadsheets, or (5) Recalculating formulas"
license: Proprietary. LICENSE.txt has complete terms
---

# Excel电子表格处理专家 Real Skill

## 一句话说明

专业的 Excel 处理工具，强制零公式错误标准，支持数据分析、报表生成和财务建模。

**Excel Spreadsheet Mastery** - Zero-error formula standard, data analysis, automated reporting, financial modeling with color coding.

---

## 使用场景

**适用**：
- 数据分析和统计报告
- 自动化生成 Excel 报表
- 财务预测模型（色彩编码）
- 批量处理多个 Excel 文件
- 数据清洗和格式转换

**不适用**：
- 实时协同编辑（用 Google Sheets / use Google Sheets for real-time collaboration）
- 复杂宏操作（用 VBA / use VBA for complex macros）

---

## 核心流程

```
用户需求 → 判断任务类型 → 选择工具 → 执行操作 → 公式重算 → 验证输出
User request → Task classification → Tool selection → Execute → Recalc formulas → Validate
```

**任务类型判断 (Task Classification)**：

### Data Analysis (数据分析)
- **Simple analysis** → pandas (read, analyze, visualize)
- **Statistical reports** → pandas + matplotlib/seaborn
- **Data cleaning** → pandas transformations

### Spreadsheet Creation (创建电子表格)
- **With formulas** → openpyxl + recalc.py (MANDATORY)
- **Without formulas** → pandas (faster)
- **Financial models** → openpyxl + color coding + recalc.py

### Editing Existing (编辑已有文件)
- **Data updates** → openpyxl (preserves formulas)
- **Template modifications** → Match existing format EXACTLY

⚠️ **CRITICAL**: After using formulas, MUST run `python scripts/recalc.py output.xlsx`

---

## 任务完成标准

**必须满足**（缺一不可）：
- ✅ **零公式错误 (Zero Formula Errors)**（#REF!, #DIV/0!, #VALUE!, #N/A, #NAME? = 0）
- Excel 文件可正常打开
- 数据完整准确
- 格式规范（数字格式、对齐、边框）
- 财务模型使用行业标准色彩编码（如适用）

**质量评级**：
- ⭐⭐⭐⭐⭐ 优秀 - 零错误 + 色彩编码 + 完整文档
- ⭐⭐⭐ 及格 - 零错误 + 数据正确
- ⭐ 失败 - 存在公式错误

---

## 参考资料（供 AI 使用）

| 类型 | 路径 | 说明 |
|-----|------|------|
| 核心文档 | `docs/00-SKILL-完整操作指南.md` | Complete Excel processing guide (~300 lines) |
| 工具脚本 | `scripts/recalc.py` | Formula recalculation script (requires LibreOffice) |

---

## 关键原则（AI 必读 / Critical Principles）

### 1. ⚠️ Zero Formula Errors (零公式错误原则 - MANDATORY)
**Every Excel file MUST be delivered with ZERO formula errors**:
- No #REF! (invalid references)
- No #DIV/0! (division by zero)
- No #VALUE! (wrong data type)
- No #N/A (lookup not found)
- No #NAME? (unrecognized formula name)

**Verification workflow**:
```bash
python scripts/recalc.py output.xlsx
# Check JSON output: status should be "success"
# If "errors_found", fix and recalculate
```

### 2. Use Formulas, Not Hardcoded Values (使用公式而非硬编码)
**CRITICAL**: Always use Excel formulas instead of calculating in Python.

❌ **WRONG - Hardcoding**:
```python
total = df['Sales'].sum()
sheet['B10'] = total  # Hardcodes 5000
```

✅ **CORRECT - Using Formulas**:
```python
sheet['B10'] = '=SUM(B2:B9)'  # Dynamic formula
```

**Why**: Spreadsheet remains dynamic and updateable when source data changes.

### 3. Financial Modeling Color Coding (财务建模色彩编码)
**Industry-standard color conventions** (unless user specifies otherwise):

```python
from openpyxl.styles import Font

# Blue text (0000FF): User inputs and assumptions
ws['A1'].font = Font(color='0000FF')

# Black text (000000): ALL formulas and calculations
ws['B1'].font = Font(color='000000')

# Green text (008000): Links within same workbook
ws['C1'].font = Font(color='008000')

# Red text (FF0000): External file links
ws['D1'].font = Font(color='FF0000')

# Yellow background (FFFF00): Key assumptions
from openpyxl.styles import PatternFill
ws['E1'].fill = PatternFill(start_color='FFFF00', fill_type='solid')
```

### 4. Number Formatting Standards (数字格式标准)
```python
from openpyxl.styles import numbers

# Years: Format as text strings "2024" (not numbers)
ws['A1'].number_format = '@'  # Text format
ws['A1'] = "'2024"  # Force text

# Currency: $#,##0 + specify units in header
ws['B1'].number_format = '$#,##0'

# Zeros: Format as "-"
ws['C1'].number_format = '$#,##0;($#,##0);"-"'

# Percentages: 0.0% (one decimal)
ws['D1'].number_format = '0.0%'

# Multiples: 0.0x for valuation multiples
ws['E1'].number_format = '0.0"x"'

# Negative numbers: Use parentheses (123) not minus -123
ws['F1'].number_format = '#,##0;(#,##0)'
```

### 5. Preserve Existing Templates (保留现有模板)
When modifying existing files:
- Study and EXACTLY match existing format, style, conventions
- Never impose standardized formatting on files with established patterns
- Existing template conventions ALWAYS override these guidelines

---

## 快速命令参考 (Quick Commands)

### Data Analysis with Pandas (数据分析)
```python
import pandas as pd

# Read Excel (single sheet or all)
df = pd.read_excel('file.xlsx')  # First sheet
all_sheets = pd.read_excel('file.xlsx', sheet_name=None)  # All sheets as dict

# Analyze
df.head()       # Preview data
df.info()       # Column info
df.describe()   # Statistics
df.groupby('Category')['Sales'].sum()  # Group by

# Write Excel
df.to_excel('output.xlsx', index=False)
```

### Create Excel with Formulas (创建带公式的 Excel)
```python
from openpyxl import Workbook
from openpyxl.styles import Font

wb = Workbook()
ws = wb.active

# Data with formulas
ws['A1'] = 'Revenue'
ws['A2'] = 1000
ws['A3'] = 1200
ws['A4'] = 'Total'
ws['B4'] = '=SUM(A2:A3)'  # Use formula, not hardcoded value

# Color coding
ws['A2'].font = Font(color='0000FF')  # Blue: User input
ws['B4'].font = Font(color='000000')  # Black: Formula

wb.save('output.xlsx')
```

### Recalculate Formulas (公式重算 - MANDATORY)
```bash
# After saving file with formulas
python scripts/recalc.py output.xlsx

# Output JSON shows:
# - status: "success" or "errors_found"
# - error_summary: Details of any errors
# - total_errors: Count of formula errors
```

### Financial Model with Color Coding (财务建模)
```python
from openpyxl import Workbook
from openpyxl.styles import Font, PatternFill

wb = Workbook()
ws = wb.active

# Headers
ws['A1'] = 'Year'
ws['B1'] = 'Revenue ($mm)'
ws['C1'] = 'Growth %'

# Data with color coding
ws['A2'] = "'2024"  # Force text for year
ws['B2'] = 100
ws['B2'].font = Font(color='0000FF')  # Blue: User input

ws['C2'] = 0.15  # Growth assumption
ws['C2'].font = Font(color='0000FF')
ws['C2'].fill = PatternFill(start_color='FFFF00', fill_type='solid')  # Yellow: Key assumption
ws['C2'].number_format = '0.0%'

# Formulas
ws['B3'] = '=B2*(1+C2)'  # Revenue calculation
ws['B3'].font = Font(color='000000')  # Black: Formula

wb.save('financial_model.xlsx')

# MUST recalculate
import subprocess
subprocess.run(['python', 'scripts/recalc.py', 'financial_model.xlsx'])
```

### Error Handling (错误处理)
```python
import subprocess
import json

# Run recalc and check results
result = subprocess.run(
    ['python', 'scripts/recalc.py', 'output.xlsx'],
    capture_output=True,
    text=True
)

data = json.loads(result.stdout)

if data['status'] == 'errors_found':
    print(f"Found {data['total_errors']} errors:")
    for error_type, count in data['error_summary'].items():
        print(f"  {error_type}: {count}")
    # Fix errors and recalculate
else:
    print("✅ Zero formula errors - file ready!")
```

---

## 依赖安装 (Dependencies Installation)

### Python Dependencies
```bash
pip install -r requirements.txt
# Includes: openpyxl, pandas, xlrd, openpyxl
```

### System Dependency (LibreOffice)
```bash
# macOS
brew install --cask libreoffice

# Ubuntu/Debian
sudo apt-get install libreoffice

# Windows: Download from https://www.libreoffice.org/
```

**Note**: recalc.py automatically configures LibreOffice on first run.

---

## 常见场景示例 (Common Scenarios)

### Scenario 1: Create Sales Report with Auto-Total (创建带自动汇总的销售报告)
```python
import pandas as pd
from openpyxl import load_workbook
from openpyxl.styles import Font

# Step 1: Create data with pandas
data = {
    'Product': ['A', 'B', 'C'],
    'Q1': [1000, 800, 1200],
    'Q2': [1100, 850, 1300],
    'Q3': [1050, 900, 1250],
    'Q4': [1200, 950, 1400]
}
df = pd.DataFrame(data)
df.to_excel('sales.xlsx', index=False, startrow=1)

# Step 2: Add formulas with openpyxl
wb = load_workbook('sales.xlsx')
ws = wb.active

# Add title
ws['A1'] = '2024 Sales Report ($000s)'
ws['A1'].font = Font(bold=True, size=14)

# Add totals row
last_row = len(df) + 2
ws[f'A{last_row}'] = 'Total'
ws[f'A{last_row}'].font = Font(bold=True)

for col in ['B', 'C', 'D', 'E']:
    ws[f'{col}{last_row}'] = f'=SUM({col}2:{col}{last_row-1})'
    ws[f'{col}{last_row}'].font = Font(color='000000', bold=True)

wb.save('sales.xlsx')

# Step 3: Recalculate
import subprocess
subprocess.run(['python', 'scripts/recalc.py', 'sales.xlsx'])
```

### Scenario 2: 3-Year Financial Projection (三年财务预测)
```python
from openpyxl import Workbook
from openpyxl.styles import Font, PatternFill, Alignment

wb = Workbook()
ws = wb.active

# Headers
headers = ['Metric', '2024', '2025', '2026']
for col, header in enumerate(headers, start=1):
    ws.cell(1, col, header)
    ws.cell(1, col).font = Font(bold=True)
    ws.cell(1, col).alignment = Alignment(horizontal='center')

# Assumptions (Blue + Yellow background)
ws['A2'] = 'Base Revenue'
ws['B2'] = 10000
ws['B2'].font = Font(color='0000FF')
ws['B2'].fill = PatternFill(start_color='FFFF00', fill_type='solid')

ws['A3'] = 'Growth Rate'
ws['B3'] = 0.15
ws['B3'].font = Font(color='0000FF')
ws['B3'].fill = PatternFill(start_color='FFFF00', fill_type='solid')
ws['B3'].number_format = '0.0%'

# Calculations (Black formulas)
ws['A5'] = 'Projected Revenue'
ws['B5'] = '=B2'
ws['B5'].font = Font(color='000000')

ws['C5'] = '=B5*(1+$B$3)'
ws['C5'].font = Font(color='000000')

ws['D5'] = '=C5*(1+$B$3)'
ws['D5'].font = Font(color='000000')

# Number formatting
for col in ['B', 'C', 'D']:
    ws[f'{col}5'].number_format = '$#,##0'

wb.save('projection.xlsx')

# MUST recalculate
import subprocess
subprocess.run(['python', 'scripts/recalc.py', 'projection.xlsx'])
```

### Scenario 3: Batch Process Multiple Excel Files (批量处理多个文件)
```python
import pandas as pd
import glob

all_data = []

for file in glob.glob('data/*.xlsx'):
    df = pd.read_excel(file)
    df['source_file'] = file
    all_data.append(df)

combined = pd.concat(all_data, ignore_index=True)
combined.to_excel('combined_report.xlsx', index=False)

print(f"Processed {len(all_data)} files, total {len(combined)} rows")
```
