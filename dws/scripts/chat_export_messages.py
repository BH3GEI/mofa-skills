#!/usr/bin/env python3
"""
导出群聊消息到 JSON 文件（从指定时间点拉取）

用法:
    python chat_export_messages.py \
        --group <openconversation_id> \
        --time "2026-03-10 00:00:00" \
        --output messages.json

    python chat_export_messages.py \
        --query "项目冲刺" \
        --time "2026-03-10 00:00:00" \
        --no-forward --limit 100
"""

import sys
import json
import subprocess
import argparse
from typing import List, Any, Optional


def run_dws(
    args: List[str], dry_run: bool = False,
) -> Optional[Any]:
    cmd = ['dws'] + args
    if dry_run:
        print(f"[dry-run] {' '.join(cmd)}")
        return None
    try:
        result = subprocess.run(
            cmd, capture_output=True, text=True, timeout=120
        )
        if result.returncode != 0:
            print(f"错误：{result.stderr.strip()}", file=sys.stderr)
            return None
        return json.loads(result.stdout)
    except (subprocess.TimeoutExpired, json.JSONDecodeError,
            FileNotFoundError) as e:
        print(f"错误：{e}", file=sys.stderr)
        return None


def search_group(
    query: str, dry_run: bool = False,
) -> Optional[str]:
    data = run_dws([
        'chat', 'search',
        '--query', query, '--format', 'json',
    ], dry_run=dry_run)
    if dry_run:
        return '<CONV_ID>'
    if not data:
        return None
    groups = (data if isinstance(data, list)
              else data.get('items', data.get('groups', [])))
    if not groups:
        print(f"未找到群聊: {query}")
        return None
    g = groups[0]
    name = g.get('title') or g.get('name', '未知')
    conv_id = g.get('openConversationId') or g.get('id')
    print(f"  找到群聊: {name} ({conv_id})")
    return conv_id


def main():
    parser = argparse.ArgumentParser(
        description='导出群聊消息到 JSON'
    )
    parser.add_argument('--group', help='群聊 openconversation_id')
    parser.add_argument('--query', help='按群名搜索')
    parser.add_argument(
        '--time', required=True,
        help='起始时间 yyyy-MM-dd HH:mm:ss',
    )
    parser.add_argument(
        '--no-forward', action='store_true',
        help='拉给定时间之前的消息 (默认拉给定时间之后)',
    )
    parser.add_argument(
        '--limit', type=int, default=0,
        help='返回条数 (不传则不限制)',
    )
    parser.add_argument('--output', default='', help='输出文件')
    parser.add_argument('--dry-run', action='store_true')
    args = parser.parse_args()

    conv_id = args.group
    if not conv_id:
        if not args.query:
            print('错误：需要 --group 或 --query 参数')
            sys.exit(1)
        print(f'🔍 搜索群聊: {args.query}')
        conv_id = search_group(args.query, args.dry_run)
        if not conv_id and not args.dry_run:
            sys.exit(1)

    print(f'📥 拉取消息 (起始: {args.time})...')
    cmd_args = [
        'chat', 'message', 'list',
        '--group', conv_id or '<CONV_ID>',
        '--time', args.time,
        '--format', 'json',
    ]
    if args.no_forward:
        cmd_args.append('--forward=false')
    if args.limit > 0:
        cmd_args.extend(['--limit', str(args.limit)])
    data = run_dws(cmd_args, dry_run=args.dry_run)

    if args.dry_run:
        return
    if not data:
        print('未拉取到消息')
        return

    messages = (data if isinstance(data, list)
                else data.get('messages', data.get('result', [])))

    if args.output:
        with open(args.output, 'w', encoding='utf-8') as f:
            json.dump(messages, f, ensure_ascii=False, indent=2)
        print(f"  ✓ 已导出 {len(messages)} 条消息到 {args.output}")
    else:
        for m in messages:
            sender = m.get('senderNick') or m.get('sender', '未知')
            text = m.get('text') or m.get('content', '')
            time_str = m.get('createAt') or m.get('time', '')
            print(f"  [{time_str}] {sender}: {text[:80]}")
        print(f"\n合计: {len(messages)} 条消息")


if __name__ == '__main__':
    main()
