---
name: mofa-vercel-web
description: "Generate websites with Claude Code and deploy to Vercel in one command. Supports static HTML, Next.js, React, Vue. Triggers: deploy website, vercel deploy, 部署网站, 生成网站"
requires_bins: git, npm
requires_env: VERCEL_TOKEN
always: false
---

# Vercel 网站生成与部署

让 Claude Code 根据你的需求生成网站代码，并自动部署到 Vercel，获得在线访问链接。

## Onboarding / 开始使用

### 第一步：安装 Vercel CLI

```bash
npm install -g vercel
```

### 第二步：登录 Vercel 获取 Token

**方式 A：命令行登录（推荐）**
```bash
vercel login
# 按提示完成浏览器授权
```

**方式 B：手动获取 Token**
1. 访问 https://vercel.com/dashboard
2. Settings → Tokens → Create Token
3. 复制 Token

### 第三步：配置环境变量

```bash
export VERCEL_TOKEN="your_vercel_token_here"
```

或永久配置：
```bash
echo 'export VERCEL_TOKEN="your_token"' >> ~/.zshrc
source ~/.zshrc
```

### 第四步：验证

```bash
vercel --version
# 应显示版本号
```

---

## 使用方法

### 基础流程

告诉 Claude 你的需求，它会：

1. **分析需求** → 确定技术栈（HTML/Next.js/React/Vue）
2. **生成代码** → 创建完整项目结构
3. **本地预览** → 验证效果
4. **部署上线** → 推送到 Vercel
5. **返回链接** → 获得可访问的 URL

### 示例对话

**你:** "帮我生成一个个人博客网站，深色主题，有文章列表和关于页面"

**Claude:**
```
好的！我来为你创建一个深色主题的个人博客。

计划：
- 技术栈: Next.js + Tailwind CSS
- 功能: 首页文章列表、关于页面、响应式设计
- 部署: Vercel

开始生成代码...
✓ 创建项目结构
✓ 编写首页组件
✓ 编写关于页面
✓ 配置深色主题
✓ 本地预览通过
✓ 部署到 Vercel

🎉 部署成功！
访问链接: https://your-blog.vercel.app
```

---

## 支持的技术栈

| 技术栈 | 适用场景 | 部署方式 |
|--------|---------|---------|
| **静态 HTML** | 简单页面、展示型网站 | 直接部署 |
| **Next.js** | 博客、营销页、全栈应用 | Vercel 原生支持 |
| **React (Vite)** | 单页应用、交互式网站 | 构建后部署 |
| **Vue (Vite)** | 轻量级网站、快速原型 | 构建后部署 |
| **Astro** | 内容型网站、文档站 | 静态生成 |

---

## 生成规范

### 项目结构

Claude 生成的项目应遵循以下结构：

```
my-website/
├── README.md              # 项目说明
├── package.json           # 依赖配置
├── vercel.json            # Vercel 配置（可选）
├── public/                # 静态资源
│   └── images/
├── src/                   # 源码
│   ├── components/        # 组件
│   ├── pages/            # 页面（Next.js）
│   ├── App.jsx           # 根组件（React/Vue）
│   └── styles/           # 样式文件
└── .gitignore
```

### 代码要求

1. **单文件优先**（简单网站）
   - 生成独立的 `index.html`
   - 内联 CSS 和 JS
   - 直接部署，无需构建

2. **框架项目**（复杂网站）
   - 完整的项目配置
   - 清晰的文件结构
   - 正确的构建脚本

3. **样式规范**
   - 使用 Tailwind CSS 或现代 CSS
   - 响应式设计（移动端适配）
   - 支持深色/浅色模式（可选）

4. **性能优化**
   - 图片懒加载
   - 代码分割
   - SEO 基础配置

---

## 部署命令

### 方式 1：Vercel CLI（推荐）

```bash
# 进入项目目录
cd my-website

# 基础部署（简单情况）
vercel --token $VERCEL_TOKEN --yes

# 完整部署（避免踩坑版）
vercel --token $VERCEL_TOKEN --yes --scope your-team-name --force

# 输出示例:
# 🔗  Production: https://my-website.vercel.app
```

### 方式 2：静态 HTML 快速部署

```bash
# 创建临时目录
cd /tmp
mkdir my-static-site
cd my-static-site

# 生成 index.html（Claude 会帮你完成）
# ...

# 直接部署（静态站点无需构建）
export VERCEL_TOKEN="your_token_here"
vercel --token $VERCEL_TOKEN --yes --scope your-team-name --force

# 成功后返回链接如：https://my-static-site-xxx.vercel.app
```

### 方式 3：Git 集成

```bash
# 初始化 git
git init
git add .
git commit -m "Initial commit"

# 连接 Vercel（首次需要交互）
vercel --token $VERCEL_TOKEN

# 后续推送自动部署
git push
```

### 方式 3：Vercel API

```bash
# 创建部署
curl -X POST "https://api.vercel.com/v13/deployments" \
  -H "Authorization: Bearer $VERCEL_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "my-website",
    "files": [...],
    "projectSettings": {
      "framework": "nextjs"
    }
  }'
```

---

## 完整示例

### 示例 1：生成静态 HTML 页面

**需求:** "一个产品落地页，展示 AI 写作工具，有定价表格"

**生成代码:**
```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AI Writer - 智能写作助手</title>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-gradient-to-b from-gray-900 to-gray-800 text-white">
    <!-- Hero Section -->
    <header class="container mx-auto px-6 py-20 text-center">
        <h1 class="text-5xl font-bold mb-6">AI Writer</h1>
        <p class="text-xl text-gray-300 mb-8">让 AI 帮你写出专业内容</p>
        <button class="bg-blue-600 px-8 py-3 rounded-full hover:bg-blue-700">
            免费试用
        </button>
    </header>

    <!-- Pricing -->
    <section class="container mx-auto px-6 py-16">
        <div class="grid md:grid-cols-3 gap-8">
            <div class="bg-gray-800 p-6 rounded-lg">
                <h3 class="text-xl font-bold mb-4">免费版</h3>
                <p class="text-3xl font-bold mb-4">¥0</p>
                <ul class="space-y-2 text-gray-300">
                    <li>✓ 每月 1000 字</li>
                    <li>✓ 基础模板</li>
                </ul>
            </div>
            <!-- 更多定价... -->
        </div>
    </section>
</body>
</html>
```

**部署:**
```bash
vercel --token $VERCEL_TOKEN --yes
```

### 示例 2：生成 Next.js 项目

**需求:** "一个个人博客，支持 Markdown 文章，深色主题"

**生成项目:**
```bash
# 初始化
npx create-next-app@latest my-blog --typescript --tailwind --eslint --app --src-dir

# 安装依赖
cd my-blog
npm install gray-matter react-markdown

# 生成页面组件、样式、配置文件...
# (Claude 会完成这些)

# 构建
npm run build

# 部署
vercel --token $VERCEL_TOKEN --prod
```

---

## 高级配置

### 自定义域名

```bash
# 添加域名
vercel domains add my-website.com

# 配置 DNS 后
vercel --token $VERCEL_TOKEN
```

### 环境变量

```bash
# 设置环境变量
vercel env add API_KEY
vercel env add DATABASE_URL
```

### 部署配置 (vercel.json)

```json
{
  "version": 2,
  "name": "my-website",
  "builds": [
    {
      "src": "package.json",
      "use": "@vercel/next"
    }
  ],
  "routes": [
    {
      "src": "/(.*)",
      "dest": "/$1"
    }
  ],
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        {
          "key": "Cache-Control",
          "value": "public, max-age=3600"
        }
      ]
    }
  ]
}
```

---

## 故障排除 / 踩坑记录

### 常见问题速查

| 问题 | 解决方案 |
|------|---------|
| `VERCEL_TOKEN not found` | 检查环境变量是否设置 |
| `vercel: command not found` | 运行 `npm install -g vercel` |
| 部署失败 | 检查 `package.json` 和构建脚本 |
| 404 错误 | 检查路由配置或文件路径 |
| 样式丢失 | 确认构建输出目录正确 |

### 实战踩坑记录

#### 坑 1：missing_scope 错误

**现象：**
```json
{
  "status": "action_required",
  "reason": "missing_scope",
  "message": "Provide --scope or --team explicitly..."
}
```

**解决：**
```bash
# 添加 --scope 参数，值为你的 team name
vercel --token $VERCEL_TOKEN --yes --scope your-team-name
```

**获取 team name：**
- 错误提示中会列出可选的 teams
- 或访问 https://vercel.com/account 查看

#### 坑 2：项目链接冲突

**现象：**
提示选择 remote URL，进入交互模式：
```
? Which remote do you want to connect?
❯ git@github.com:xxx/xxx.git (origin)
  https://github.com/xxx/xxx.git (upstream)
```

**解决：**
```bash
# 清理已存在的 .vercel 配置
rm -rf .vercel

# 或用 --force 强制重新链接
vercel --token $VERCEL_TOKEN --yes --scope your-team --force
```

#### 坑 3：--name 参数已弃用

**现象：**
```
The "--name" option is deprecated (https://vercel.link/name-flag)
```

**解决：**
不指定 `--name`，Vercel 会自动从目录名创建项目。如需指定名称：
```bash
# 直接修改目录名
mv my-old-name my-new-name
cd my-new-name
vercel --token $VERCEL_TOKEN --yes
```

#### 坑 4：静态 HTML 部署配置

**现象：**
框架检测失败，提示 "No framework detected"

**解决：**
这是正常的！静态 HTML 不需要框架。确保目录结构正确：
```
my-website/
├── index.html          # 入口文件必须存在
├── css/
├── js/
└── images/
```

Vercel 会自动识别为静态站点，无需额外配置。

#### 坑 5：非交互式部署（CI/CD）

**解决：**
```bash
# 完整命令，避免任何交互
vercel deploy \
  --token $VERCEL_TOKEN \
  --yes \
  --scope your-team \
  --force \
  --prod  # 如果是生产部署
```

参数说明：
- `--yes`：跳过确认提示
- `--scope`：指定团队
- `--force`：强制重新链接项目
- `--prod`：部署到生产环境

#### 坑 6：Token 权限不足

**现象：**
```
Error: You do not have permission to access this resource
```

**解决：**
1. 检查 Token 是否过期（https://vercel.com/account/tokens）
2. 确认 Token 有对应团队的访问权限
3. 重新生成 Token 并重试

---

## 相关 Skills

### mofa-firecrawl / mofa-defuddle
**组合使用:**
```
mofa-firecrawl: 获取参考网站内容
分析设计思路
mofa-vercel-web: 生成类似网站并部署
```

### mofa-research-2.0
**组合使用:**
```
mofa-research-2.0: 研究最佳网站设计实践
mofa-vercel-web: 按照最佳实践生成网站
```

### mofa-web
**区别:**
- mofa-web: 现有网站的构建和分析
- mofa-vercel-web: 从零生成新网站并部署

---

## 成本说明

| 项目 | 费用 |
|------|------|
| Vercel Hobby 计划 | 免费（个人项目） |
| 自定义域名 | ¥50-100/年 |
| Pro 计划 | $20/月（团队） |

Hobby 计划包含：
- 无限项目数
- 100GB 带宽/月
- 6000 构建分钟/月
- Serverless Function 执行

---

## 最佳实践

1. **从简单开始**
   - 先尝试静态 HTML，再考虑框架
   - 单页面应用比多页面更简单

2. **明确需求**
   - 告诉 Claude 目标用户群体
   - 说明品牌色调偏好
   - 列出必备功能

3. **迭代优化**
   - 先部署 MVP 版本
   - 根据反馈调整
   - 逐步添加功能

4. **性能优先**
   - 图片使用 WebP 格式
   - 启用懒加载
   - 控制 JS 包大小

---

## 实战案例：部署 MoFA Showcase

**场景：** 将生成的静态 HTML 网站部署到 Vercel

**完整流程：**

```bash
# 1. 确保 Vercel CLI 已安装
npm install -g vercel

# 2. 进入网站目录
cd /tmp/mofa-showcase
ls -la
# index.html

# 3. 清理可能存在的旧配置（避免冲突）
rm -rf .vercel .git

# 4. 设置 Token 并部署
export VERCEL_TOKEN="your_token_here"
vercel --token $VERCEL_TOKEN --yes --scope bh3geis-projects --force

# 5. 查看输出
# Production: https://mofa-showcase-xxx.vercel.app
```

**遇到的坑：**
1. 第一次没加 `--scope`，报错 `missing_scope`
2. 目录里有 `.git` 配置，提示选择 remote → 用 `rm -rf .git` 清理
3. `--name` 参数已弃用 → 直接靠目录名识别

**最终结果：**
- 网站成功上线：https://mofa-showcase.vercel.app
- 静态 HTML 无需构建，直接部署
