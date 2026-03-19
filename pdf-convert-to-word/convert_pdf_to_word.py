from pdf2docx import Converter
import os

# 定义输入PDF和输出DOCX文件路径
pdf_file = '/Users/yunhuan/Downloads/测试pdf转word/曼德月度采购计划.pdf'
docx_file = '/Users/yunhuan/Downloads/测试pdf转word/曼德月度采购计划.docx'

try:
    # 检查PDF文件是否存在
    if not os.path.exists(pdf_file):
        raise FileNotFoundError(f"PDF文件不存在: {pdf_file}")
    
    # 创建一个Converter对象
    cv = Converter(pdf_file)

    # 将PDF转换为DOCX文件
    cv.convert(docx_file, start=0, end=None)

    # 关闭Converter对象
    cv.close()

    print(f"成功将 {pdf_file} 转换为 {docx_file}")

except Exception as e:
    print(f"转换过程中发生错误: {e}")