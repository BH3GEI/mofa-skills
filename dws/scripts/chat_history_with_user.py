#!/usr/bin/env python3
"""
查询与某人的单聊聊天记录

用法:
    python chat_history_with_user.py --name "张三" --time "2026-03-10 00:00:00"
    python chat_history_with_user.py --user <userId> --time "2026-03-10 00:00:00" --limit 50
    python chat_history_with_user.py --name "张三" --time "2026-03-01 00:00:00" --output history.json

工作流:
  1. 通过 --name 搜索通讯录，获取 userId（或直接传 --user）
  2. 调用 chat message list --user <userId> 拉取单聊消息
  3. 输出到终端或导出为 JSON 文件
"""

import sys
import json
import subprocess
import argparse
from typing import List, Any, Optional


def run_dws(
    args: List[str], dry_run: bool = False,
) -> Optional[Any]:
    """执行 dws 命令并解析 JSON 输出"""
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


def search_user(
    name: str, dry_run: bool = False,
) -> Optional[str]:
    """按关键词搜索用户，返回第一个匹配的 userId"""
    data = run_dws([
        'contact', 'user', 'search',
        '--keyword', name, '--format', 'json',
    ], dry_run=dry_run)
    if dry_run:
        return '<USER_ID>'
    if not data:
        return None
    # 解析返回结构
    users = data
    if isinstance(data, dict):
        users = (data.get('result', [])
                 or data.get('users', [])
                 or data.get('list', []))
    if not users or not isinstance(users, list):
        print(f"未找到用户: {name}")
        return None
    u = users[0]
    user_name = u.get('name') or u.get('nick', '未知')
    user_id = u.get('userId') or u.get('userid', '')
    print(f"  找到用户: {user_name} ({user_id})")
    return user_id


def main():
    parser = argparse.ArgumentParser(
        description='查询与某人的单聊聊天记录'
    )
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument('--name', help='按姓名搜索用户')
    group.add_argument('--user', help='直接指定 userId')
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
    parser.add_argument('--output', default='', help='导出到 JSON 文件')
    parser.add_argument('--dry-run', action='store_true')
    args = parser.parse_args()

    # 1. 获取 userId
    user_id = args.user
    if not user_id:
        print(f'🔍 搜索用户: {args.name}')
        user_id = search_user(args.name, args.dry_run)
        if not user_id and not args.dry_run:
            sys.exit(1)

    # 2. 拉取单聊消息
    print(f'📥 拉取与 {user_id} 的聊天记录 (起始: {args.time})...')
    cmd_args = [
        'chat', 'message', 'list',
        '--user', user_id or '<USER_ID>',
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

    # 3. 输出结果
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
