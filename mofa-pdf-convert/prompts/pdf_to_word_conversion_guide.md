# PDF to Word Conversion Guide for AI Agents

This guide outlines a reliable method for converting PDF files to Word documents (`.docx`) while preserving images and formatting. This process utilizes the `pdf2docx` Python library.

## 1. Goal

The primary goal is to convert a PDF file to a Word document, ensuring that images, layout, and text formatting are retained as accurately as possible.

## 2. Recommended Tool

The recommended tool for this task is the `pdf2docx` Python library. It is an open-source library that has proven to be effective for this purpose.

## 3. Conversion Process

The following steps provide a detailed walkthrough of the conversion process.

### Step 1: Check for `pdf2docx` Installation

First, check if the `pdf2docx` library is installed. Use the `pip3 show` command.

```bash
pip3 show pdf2docx
```

If the library is installed, you will see information about the package. If not, the command will likely return an error or no output.

### Step 2: Install `pdf2docx` (if necessary)

If `pdf2docx` is not installed, install it using `pip3`.

```bash
pip3 install pdf2docx
```

### Step 3: Handle Potential Version Conflicts

A known issue with `pdf2docx` is an incompatibility with newer versions of its dependency, `PyMuPDF`. This can cause an `AttributeError: 'Rect' object has no attribute 'get_area'`.

If you encounter this error, you must downgrade `PyMuPDF` to a compatible version (e.g., 1.26.4).

```bash
pip3 install PyMuPDF==1.26.4
```

### Step 4: Create a Conversion Script

Create a Python script to handle the conversion. This script will import the necessary library, define the file paths, and perform the conversion.

Here is a template for the conversion script:

```python
from pdf2docx import Converter

# Define the input PDF and output DOCX file paths
pdf_file = 'path/to/your/document.pdf'
docx_file = 'path/to/your/document.docx'

try:
    # Create a Converter object
    cv = Converter(pdf_file)

    # Convert the PDF to a DOCX file
    cv.convert(docx_file, start=0, end=None)

    # Close the Converter object
    cv.close()

    print(f"Successfully converted {pdf_file} to {docx_file}")

except Exception as e:
    print(f"An error occurred: {e}")

```

Replace `'path/to/your/document.pdf'` and `'path/to/your/document.docx'` with the actual file paths.

### Step 5: Execute the Conversion Script

Run the Python script from your terminal.

```bash
python3 /path/to/your/script/convert.py
```

The script will print a success message upon completion or an error message if something goes wrong.

### Step 6: Clean Up

After the conversion is complete, you can delete the temporary conversion script.

```bash
rm /path/to/your/script/convert.py
```

## 4. Summary of Commands

Here is a summary of the shell commands used in this process:

```bash
# Check for pdf2docx installation
pip3 show pdf2docx

# Install pdf2docx (if necessary)
pip3 install pdf2docx

# Downgrade PyMuPDF to a compatible version (if needed)
pip3 install PyMuPDF==1.26.4

# Execute the conversion script
python3 convert.py

# Remove the temporary script
rm convert.py
```

By following this guide, AI agents can efficiently and accurately convert PDF files to Word documents.
