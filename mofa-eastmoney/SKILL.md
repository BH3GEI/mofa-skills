---
name: mofa-eastmoney
description: "东方财富金融数据查询 Skill。整合金融数据、宏观经济、资讯搜索、选股四大能力。支持 A股/港股/美股/基金/债券的全方位金融数据分析。Triggers: eastmoney, 东方财富, 股票数据, 选股, 宏观经济, 金融资讯"
requires_bins: python3, pip3
requires_env: EM_API_KEY
always: false
---

# 东方财富金融数据 (MoFA-Eastmoney)

整合东方财富妙想大模型 API 的全方位金融数据查询 Skill，包含四大核心能力：

- **金融数据查询 (FinData)** - 股票、板块、指数、债券等多维金融指标
- **宏观经济数据 (MacroData)** - GDP、CPI、货币供应量等宏观指标
- **金融资讯搜索 (FinSearch)** - 新闻、公告、研报等时效性资讯
- **智能选股 (StockPick)** - A股/港股/美股/基金/ETF 筛选

## Onboarding / 开始使用

### 第一步：注册并获取 API Key

**1. 访问东方财富妙想平台**

打开 https://ai.eastmoney.com/ ，点击右上角「注册/登录」。

**2. 完成注册**

- 支持手机号注册
- 完成实名认证（如需）

**3. 获取 API Key**

```
登录后 → 控制台 → API 管理 → 创建 API Key
```

复制生成的 `EM_API_KEY`（格式类似 `em_xxxxx`）。

### 第二步：配置环境变量

**macOS / Linux:**
```bash
export EM_API_KEY="your_em_api_key_here"
```

**Windows PowerShell:**
```powershell
$env:EM_API_KEY="your_em_api_key_here"
```

**永久配置（推荐）:**

添加到 `~/.zshrc` 或 `~/.bashrc`:
```bash
echo 'export EM_API_KEY="your_em_api_key_here"' >> ~/.zshrc
source ~/.zshrc
```

### 第三步：安装依赖

```bash
pip3 install httpx pandas openpyxl
```

### 第四步：验证安装

```bash
cd /Users/yao/Desktop/code/work/mofa-org/mofa-skills/mofa-eastmoney
python3 scripts/mx_findata.py "贵州茅台的市值"
```

看到输出文件路径即表示成功！

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    MOFA-EASTMONEY 金融数据架构                              │
└─────────────────────────────────────────────────────────────────────────────┘

User Query (自然语言)
    ↓
┌─────────────────────────────────────────────────────────┐
│ 1. Intent Parser                                        │
│    ─────────────                                        │
│    • 金融数据 → MX_FinData                              │
│    • 宏观经济 → MX_MacroData                            │
│    • 资讯搜索 → MX_FinSearch                            │
│    • 选股选基 → MX_StockPick                            │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ 2. API Router                                           │
│    ──────────                                           │
│                                                         │
│    ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │
│    │  FinData    │  │  MacroData  │  │  FinSearch  │   │
│    │  金融数据    │  │  宏观经济    │  │  资讯搜索    │   │
│    └──────┬──────┘  └──────┬──────┘  └──────┬──────┘   │
│           └─────────────────┼─────────────────┘          │
│                             │                           │
│                    ┌────────┴────────┐                 │
│                    │   StockPick     │                 │
│                    │   选股选基       │                 │
│                    └────────┬────────┘                 │
└─────────────────────────────┼───────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────┐
│ 3. Eastmoney API                                        │
│    ─────────────                                        │
│    https://ai-saas.eastmoney.com/proxy/b/mcp/tool/      │
│                                                         │
│    Headers: em_api_key: {EM_API_KEY}                    │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ 4. Data Processing                                      │
│    ───────────────                                      │
│    • 数据清洗                                           │
│    • 格式转换 (Excel/CSV/TXT)                           │
│    • 中文列名映射                                       │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ 5. Output Files                                         │
│    ───────────                                          │
│    • .xlsx / .csv - 结构化数据                          │
│    • .txt - 结果说明文档                                │
└─────────────────────────────────────────────────────────┘
```

---

## 核心功能

### 1. 金融数据查询 (MX_FinData)

**支持对象:**
- 股票（A股、港股、美股）
- 板块、指数、股东
- 企业发行人、债券、非上市公司

**支持数据类型:**
- 实时行情（现价、涨跌幅、盘口）
- 量化数据（技术指标、资金流向）
- 报表数据（营收、净利润、财务比率）
- 公司基本信息、高管、主营业务、股东

**查询限制:**
- 单次最多 5 个实体
- 单次最多 3 个指标

**使用示例:**
```bash
# 基本面查询
python3 scripts/mx_findata.py "东方财富的基本面"

# 实时行情
python3 scripts/mx_findata.py "当前300059的实时买单"

# 多实体对比（最多5个）
python3 scripts/mx_findata.py "东方财富、拼多多、贵州茅台最近一年的营收、毛利、净利"
```

**输出文件:**
- `MX_FinData_<id>.xlsx` - 多 sheet Excel
- `MX_FinData_<id>_description.txt` - 结果说明

---

### 2. 宏观经济数据 (MX_MacroData)

**支持指标:**
- GDP、CPI、PPI、PMI、失业率
- M1/M2 货币供应量、社融规模
- 国债利率、汇率
- 商品价格（黄金、白银、原油、铜等）

**输入约束（重要）:**
- ✅ 必须指定具体省/市/国家全称
- ✅ 必须指定具体商品名称
- ✅ 必须转换为绝对时间格式 `YYYY-MM-DD`
- ✅ 必须指定具体指标名称

**使用示例:**
```bash
# ✅ 正确示例
python3 scripts/mx_macrodata.py "中国GDP同比增速"
python3 scripts/mx_macrodata.py "上海市、江苏省、浙江省的GDP数据"
python3 scripts/mx_macrodata.py "氧化镨钕、铜、铝的现货价格走势"

# ❌ 错误示例
python3 scripts/mx_macrodata.py "华东五市GDP"           # 模糊地域
python3 scripts/mx_macrodata.py "稀土价格走势"          # 模糊商品
python3 scripts/mx_macrodata.py "中国经济数据"          # 宏观泛指
```

**输出文件:**
- `macro_data_<query>_年.csv`
- `macro_data_<query>_季.csv`
- `macro_data_<query>_description.txt`

---

### 3. 金融资讯搜索 (MX_FinSearch)

**适用场景:**
- 最新新闻与政策动态
- 公司公告与事件跟踪
- 券商研报与市场解读
- 宏观事件对市场影响分析

**使用示例:**
```bash
# 个股资讯
python3 scripts/mx_finsearch.py "格力电器最新研报与公告"

# 板块资讯
python3 scripts/mx_finsearch.py "商业航天板块近期新闻"

# 宏观风险
python3 scripts/mx_finsearch.py "美联储加息对A股影响"

# 仅输出不保存
python3 scripts/mx_finsearch.py "新能源板块政策" --no-save
```

**输出文件:**
- `financial_search_<query>.txt` - 资讯正文

---

### 4. 智能选股 (MX_StockPick)

**支持类型:**
- A股、港股、美股
- 基金、ETF、可转债
- 板块

**筛选条件:**
- 财务指标：股价、市值、市盈率、涨跌幅
- 技术指标：连续上涨、突破均线等
- 主营业务、行业板块
- 高管信息、股东信息、龙虎榜

**使用示例:**
```bash
# 选A股
python3 scripts/mx_stockpick.py --query "股价大于100元的股票" --select-type "A股"

# 选港股
python3 scripts/mx_stockpick.py --query "港股的科技龙头" --select-type "港股"

# 选基金
python3 scripts/mx_stockpick.py --query "白酒主题基金近一年收益排名" --select-type "基金"

# 选ETF
python3 scripts/mx_stockpick.py --query "规模超2亿的电力ETF" --select-type "ETF"

# 选可转债
python3 scripts/mx_stockpick.py --query "价格低于110元、溢价率超5个点的可转债" --select-type "可转债"
```

**输出文件:**
- `MX_StockPick_<query>.csv` - 全量数据（中文列名）
- `MX_StockPick_<query>_description.txt` - 数据说明

---

## 快速参考

| 功能 | 命令 | 输出 |
|------|------|------|
| 金融数据 | `python3 scripts/mx_findata.py "{query}"` | .xlsx + .txt |
| 宏观经济 | `python3 scripts/mx_macrodata.py "{query}"` | .csv + .txt |
| 资讯搜索 | `python3 scripts/mx_finsearch.py "{query}"` | .txt |
| 选股选基 | `python3 scripts/mx_stockpick.py --query "{query}" --select-type "{类型}"` | .csv + .txt |

---

## 代码调用示例

```python
import asyncio
from pathlib import Path
from scripts.mx_findata import query_financial_data
from scripts.mx_macrodata import query_macro_data
from scripts.mx_finsearch import query_financial_news
from scripts.mx_stockpick import query_select_stock

async def main():
    # 1. 查询金融数据
    fin_result = await query_financial_data(
        query="贵州茅台近期走势如何",
        output_dir=Path("workspace/MX_FinData")
    )
    print(f"Excel: {fin_result['file_path']}")
    print(f"行数: {fin_result['row_count']}")

    # 2. 查询宏观经济
    macro_result = await query_macro_data(
        query="中国近五年GDP",
        output_dir=Path("workspace/macro_data")
    )
    print(f"CSV: {macro_result['csv_paths']}")

    # 3. 搜索金融资讯
    news_result = await query_financial_news(
        query="新能源板块近期政策与龙头公司动态",
        output_dir=Path("workspace/financial_search"),
        save_to_file=True
    )
    print(f"资讯: {news_result['content'][:500]}...")

    # 4. 选股
    stock_result = await query_select_stock(
        query="A股半导体板块市值前20",
        selectType="A股",
        output_dir=Path("workspace/MX_StockPick")
    )
    print(f"选中 {stock_result['row_count']} 只股票")

asyncio.run(main())
```

---

## 环境变量

| 变量 | 说明 | 必填 |
|------|------|------|
| `EM_API_KEY` | 东方财富 API Key | ✅ |
| `MX_StockPick_OUTPUT_DIR` | 选股输出目录 | 可选 |

---

## 常见问题

**Q: 如何获取 EM_API_KEY？**
A: 访问 https://ai.eastmoney.com/ → 注册登录 → 控制台 → API 管理 → 创建 API Key

**Q: 提示 "EM API KEY REQUIRED" 怎么办？**
A: 确保已设置环境变量：`export EM_API_KEY="your_key"`

**Q: 金融数据查询有数量限制吗？**
A: 单次查询最多 5 个实体 + 3 个指标，超限会自动截取前 5 个实体和前 3 个指标

**Q: 宏观经济查询有什么限制？**
A: 必须使用明确的地域、商品、时间、指标名称，禁止模糊表述

**Q: 输出文件在哪里？**
A: 默认在 `workspace/` 目录下，每个功能有独立的子目录

---

## 相关 Skills

### mofa-research-2.0
**组合使用:**
```
mofa-research-2.0: 发现投资机会
mofa-eastmoney (StockPick): 筛选标的
mofa-eastmoney (FinData): 深度分析财务数据
mofa-research-2.0: 生成投资报告
```

### mofa-news
**区别:**
- mofa-news: 通用新闻聚合（RSS）
- mofa-eastmoney (FinSearch): 专业金融资讯（东方财富）

### mofa-public-apis
**组合使用:**
```
mofa-public-apis: 发现其他金融 API
mofa-eastmoney: 获取东方财富专业数据
```

---

## API 文档

- 官方平台: https://ai.eastmoney.com/
- API 端点: `https://ai-saas.eastmoney.com/proxy/b/mcp/tool/`
- 认证方式: Header `em_api_key: {EM_API_KEY}`

---

## 合规说明

- 禁止硬编码 API Key，必须使用环境变量
- API Key 按敏感信息处理，不在日志中泄露
- 检索失败时不得编造事实
- 输出应保持可追溯、可审计
