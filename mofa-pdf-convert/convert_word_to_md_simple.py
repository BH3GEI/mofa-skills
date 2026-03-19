import docx
import os
from docx.table import Table
from docx.oxml.table import CT_Tbl
from docx.oxml.text.paragraph import CT_P
from docx.text.paragraph import Paragraph


def is_table(element):
    """检查元素是否为表格"""
    return isinstance(element, CT_Tbl)


def is_paragraph(element):
    """检查元素是否为段落"""
    return isinstance(element, CT_P)


def get_cell_text(cell):
    """获取单元格文本"""
    texts = []
    for paragraph in cell.paragraphs:
        texts.append(paragraph.text)
    return ' '.join(texts)


def convert_docx_to_md(docx_path):
    """
    将Word文档转换为Markdown格式
    """
    doc = docx.Document(docx_path)
    
    md_content = []
    
    # 遍历文档中的所有元素
    for element in doc.element.body:
        if is_paragraph(element):
            # 处理段落
            paragraph = Paragraph(element, doc)
            text = paragraph.text.strip()
            if text:
                md_content.append(text)
                md_content.append('\n\n')
        elif is_table(element):
            # 处理表格
            table = Table(element, doc.part)
            
            # 添加表头
            if len(table.rows) > 0:
                # 获取第一行作为表头
                header = table.rows[0]
                header_cells = [get_cell_text(cell) for cell in header.cells]
                md_content.append('| ' + ' | '.join(header_cells) + ' |\n')
                
                # 添加分隔行
                md_content.append('| ' + ' | '.join(['---'] * len(header_cells)) + ' |\n')
                
                # 添加其余行
                for i in range(1, len(table.rows)):
                    row = table.rows[i]
                    row_cells = [get_cell_text(cell) for cell in row.cells]
                    md_content.append('| ' + ' | '.join(row_cells) + ' |\n')
            
            md_content.append('\n\n')
    
    return ''.join(md_content)


def main():
    # 获取当前目录
    current_dir = os.getcwd()
    
    # 查找所有.docx文件（排除临时文件）
    all_files = os.listdir(current_dir)
    docx_files = [f for f in all_files if f.endswith('.docx') and not f.startswith('~$')]
    
    if not docx_files:
        print("当前目录下没有找到.docx文件")
        return
    
    print(f"找到 {len(docx_files)} 个Word文档: {docx_files}")
    
    # 转换每个Word文档
    for docx_file in docx_files:
        docx_path = os.path.join(current_dir, docx_file)
        base_name = os.path.splitext(os.path.basename(docx_path))[0]
        md_file_path = os.path.join(current_dir, f"{base_name}.md")
        
        print(f"正在将 {docx_file} 转换为 Markdown...")
        
        try:
            markdown_content = convert_docx_to_md(docx_path)
            
            with open(md_file_path, 'w', encoding='utf-8') as f:
                f.write(markdown_content)
            
            print(f"转换完成：{md_file_path}")
            
        except Exception as e:
            print(f"转换失败 {docx_file}: {e}")


if __name__ == "__main__":
    main()