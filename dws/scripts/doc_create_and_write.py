#!/usr/bin/env python3
"""
在指定目录创建文档并写入 Markdown 内容（一键完成）

用法:
    python doc_create_and_write.py \
        --name "项目周报" \
        --content "# 本周总结\n\n## 完成事项\n- 任务A"

    python doc_create_and_write.py \
        --name "会议纪要" \
        --content-file notes.md

    python doc_create_and_write.py \
        --name "知识库文档" --content "# 内容" --folder FOLDER_ID

    python doc_create_and_write.py --name "test" --content "hello" --dry-run
"""

import sys
import json
import subprocess
import argparse
from pathlib import Path
from typing import List, Any, Optional


def run_dws(
    args: List[str], dry_run: bool = False,
) -> Optional[Any]:
    cmd = ['dws'] + args
    if dry_run:
        print(f"[dry-run] {' '.join(cmd)}")
        return {'dry_run': True}
    try:
        result = subprocess.run(
            cmd, capture_output=True, text=True, timeout=60
        )
        if result.returncode != 0:
            print(f"  ✗ 错误：{result.stderr.strip()}")
            return None
        return json.loads(result.stdout)
    except (subprocess.TimeoutExpired, json.JSONDecodeError,
            FileNotFoundError) as e:
        print(f"  ✗ 错误：{e}")
        return None


def main():
    parser = argparse.ArgumentParser(
        description='创建文档并写入内容'
    )
    parser.add_argument('--name', required=True, help='文档名称')
    parser.add_argument('--content', default='', help='Markdown 内容')
    parser.add_argument('--content-file', default='', help='内容文件')
    parser.add_argument('--folder', default='', help='目标文件夹 ID 或 URL')
    parser.add_argument('--workspace', default='', help='目标知识库 ID')
    parser.add_argument(
        '--mode', default='append', choices=['overwrite', 'append'],
        help='写入模式: overwrite=覆盖, append=追加 (默认 append)',
    )
    parser.add_argument('--dry-run', action='store_true')
    args = parser.parse_args()

    content = args.content
    if args.content_file:
        p = Path(args.content_file)
        if not p.exists():
            print(f"错误：文件不存在: {p}")
            sys.exit(1)
        content = p.read_text(encoding='utf-8')
    if not content:
        print('错误：需要 --content 或 --content-file')
        sys.exit(1)
    if len(content) > 50000:
        print('警告：内容较长，可能需要分段写入')

    create_args = ['doc', 'create', '--name', args.name, '--format', 'json']
    if args.folder:
        create_args.extend(['--folder', args.folder])
    if args.workspace:
        create_args.extend(['--workspace', args.workspace])

    print(f'\n📝 创建文档: {args.name}')
    create_data = run_dws(create_args, dry_run=args.dry_run)

    node_id = None
    if not args.dry_run:
        if not create_data:
            sys.exit(1)
        node_id = (create_data.get('nodeId')
                   or create_data.get('dentryUuid')
                   or create_data.get('id', ''))
        print(f"  ✓ 文档已创建 (ID: {node_id})")

    mode_label = '追加' if args.mode == 'append' else '覆盖'
    print(f'\n✍️  写入内容 (模式: {mode_label})...')
    write_data = run_dws([
        'doc', 'update',
        '--node', node_id or '<NODE_ID>',
        '--markdown', content,
        '--mode', args.mode,
        '--format', 'json',
    ], dry_run=args.dry_run)

    if write_data:
        print(f"  ✓ 内容已写入 ({len(content)} 字符)")
    print('\n✅ 完成!')


if __name__ == '__main__':
    main()
