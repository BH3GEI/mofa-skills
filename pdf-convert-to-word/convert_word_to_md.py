import docx
import os


def word_to_markdown(docx_path):
    """
    将Word文档(.docx)转换为Markdown格式
    """
    # 加载Word文档
    try:
        doc = docx.Document(docx_path)
    except Exception as e:
        print(f"错误：无法加载文档 {docx_path} - {e}")
        return None

    markdown_content = []
    
    # 处理文档中的每个段落和元素
    for element in doc.element.body:
        # 检查元素类型并转换为相应的Markdown格式
        if element.tag.endswith('p'):  # 段落
            paragraph = docx.text.paragraph.Paragraph(element, doc.styles)
            text = paragraph.text.strip()
            if text:
                # 检查标题样式
                style = paragraph.style.name
                if style.startswith('Heading'):
                    level = style.split(' ')[1] if ' ' in style else '1'
                    try:
                        level = int(level)
                    except ValueError:
                        level = 1
                    markdown_content.append(f"{'#' * level} {text}\n")
                else:
                    markdown_content.append(f"{text}\n")
        elif element.tag.endswith('tbl'):  # 表格
            table = docx.table.Table(element, doc.part)
            # 转换表格为Markdown格式
            markdown_content.append("\n| ")
            # 添加表头
            header_cells = [cell.text for cell in table.rows[0].cells]
            markdown_content.append(" | ".join(header_cells))
            markdown_content.append(" |\n")
            
            # 添加分隔行
            markdown_content.append("| ")
            markdown_content.append(" | ".join(['---' for _ in header_cells]))
            markdown_content.append(" |\n")
            
            # 添加其他行
            for i in range(1, len(table.rows)):
                cells = [cell.text for cell in table.rows[i].cells]
                markdown_content.append("| ")
                markdown_content.append(" | ".join(cells))
                markdown_content.append(" |\n")
    
    return "".join(markdown_content)


def convert_docx_to_md(docx_file_path):
    """
    转换单个docx文件到markdown
    """
    # 获取文件名（不含扩展名）
    base_name = os.path.splitext(os.path.basename(docx_file_path))[0]
    md_file_path = os.path.join(os.path.dirname(docx_file_path), f"{base_name}.md")
    
    print(f"正在将 {docx_file_path} 转换为 {md_file_path}")
    
    markdown_content = word_to_markdown(docx_file_path)
    
    if markdown_content:
        with open(md_file_path, 'w', encoding='utf-8') as f:
            f.write(markdown_content)
        print(f"转换完成：{md_file_path}")
        return md_file_path
    else:
        print(f"转换失败：{docx_file_path}")
        return None


def main():
    # 获取当前目录
    current_dir = os.getcwd()
    
    # 查找所有.docx文件
    docx_files = [f for f in os.listdir(current_dir) if f.endswith('.docx')]
    
    if not docx_files:
        print("当前目录下没有找到.docx文件")
        return
    
    print(f"找到 {len(docx_files)} 个Word文档: {docx_files}")
    
    # 转换每个Word文档
    for docx_file in docx_files:
        docx_path = os.path.join(current_dir, docx_file)
        convert_docx_to_md(docx_path)


if __name__ == "__main__":
    main()