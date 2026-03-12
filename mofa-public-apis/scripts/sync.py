#!/usr/bin/env python3
"""
Sync public-apis data from GitHub to local JSON cache.
Usage: python3 scripts/sync.py
"""

import re
import json
import urllib.request
from datetime import datetime

README_URL = "https://raw.githubusercontent.com/public-apis/public-apis/master/README.md"
OUTPUT_FILE = "../apis.json"

def fetch_readme():
    """Fetch README from GitHub."""
    print(f"Fetching {README_URL}...")
    with urllib.request.urlopen(README_URL) as response:
        return response.read().decode('utf-8')

def parse_apis(content):
    """Parse markdown table to API list."""
    category_pattern = r'### ([^\n]+)\nAPI \| Description \| Auth \| HTTPS \| CORS.*?\n\|.*?\n((?:\|.*?\n)+)'
    matches = re.findall(category_pattern, content, re.DOTALL)

    apis = []
    categories = set()

    for category, table_content in matches:
        category = category.strip()
        categories.add(category)

        rows = re.findall(r'\| \[(.*?)\]\((.*?)\) \| (.*?) \| (.*?) \| (.*?) \| (.*?) \|?', table_content)

        for row in rows:
            name, url, description, auth, https, cors = row

            auth_clean = auth.strip().replace('`', '') if auth.strip() else 'none'
            if auth_clean in ['No', 'no', '']:
                auth_clean = 'none'

            https_clean = https.strip().lower() == 'yes'
            cors_clean = cors.strip().lower()
            if cors_clean not in ['yes', 'no']:
                cors_clean = 'unknown'

            apis.append({
                'name': name.strip(),
                'description': description.strip(),
                'url': url.strip(),
                'category': category,
                'auth': auth_clean,
                'https': https_clean,
                'cors': cors_clean
            })

    return apis, sorted(categories)

def main():
    content = fetch_readme()
    apis, categories = parse_apis(content)

    output = {
        'metadata': {
            'source': 'https://github.com/public-apis/public-apis',
            'total_apis': len(apis),
            'total_categories': len(categories),
            'categories': categories,
            'updated_at': datetime.now().strftime('%Y-%m-%d')
        },
        'apis': apis
    }

    with open(OUTPUT_FILE, 'w', encoding='utf-8') as f:
        json.dump(output, f, indent=2, ensure_ascii=False)

    print(f"✓ Synced {len(apis)} APIs, {len(categories)} categories")
    print(f"✓ Saved to {OUTPUT_FILE}")

if __name__ == '__main__':
    main()
