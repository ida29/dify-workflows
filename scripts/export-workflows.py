#!/usr/bin/env python3
"""
Dify Workflow Exporter

このスクリプトは、Dify環境から全てのワークフローをエクスポートし、
ディレクトリ構造で保存します。
"""
import os
import sys
import json
import argparse
import requests
from pathlib import Path
from typing import List, Dict, Any


def get_apps_list(dify_url: str, api_key: str) -> List[Dict[str, Any]]:
    """
    Difyから全アプリのリストを取得
    api_keyはアクセストークン（BearerまたはCookie形式）
    """
    url = f"{dify_url}/console/api/apps"

    # api_keyの形式を判定（Cookie形式か、トークンのみか）
    # JWTトークン（ey...で始まる）が末尾に=を含む場合をCookieと誤判定しないようにする
    is_cookie = False
    if ';' in api_key:
        is_cookie = True
    elif '=' in api_key and not api_key.strip().startswith('ey'):
        is_cookie = True

    if is_cookie:
        # Cookie形式
        headers = {
            'Cookie': api_key,
            'Content-Type': 'application/json'
        }
        # CSRFトークンをヘッダーに追加（Cookieから抽出）
        try:
            for part in api_key.split(';'):
                if '=' in part:
                    key, value = part.strip().split('=', 1)
                    if 'csrf_token' in key:
                        headers['X-CSRF-Token'] = value
                        break
        except Exception:
            pass
    else:
        # Bearer token形式
        headers = {
            'Authorization': f'Bearer {api_key}',
            'Content-Type': 'application/json'
        }
        print("Using Bearer token authentication")

    all_apps = []
    page = 1

    while True:
        response = requests.get(
            url,
            headers=headers,
            params={'page': page, 'limit': 100},
            timeout=30
        )
        response.raise_for_status()

        data = response.json()
        apps = data.get('data', [])

        if not apps:
            break

        all_apps.extend(apps)

        # ページネーション確認
        if not data.get('has_more', False):
            break

        page += 1

    return all_apps


def export_app_dsl(dify_url: str, api_key: str, app_id: str) -> str:
    """
    特定アプリのDSLをエクスポート
    """
    url = f"{dify_url}/console/api/apps/{app_id}/export"

    # api_keyの形式を判定（Cookie形式か、トークンのみか）
    # JWTトークン（ey...で始まる）が末尾に=を含む場合をCookieと誤判定しないようにする
    is_cookie = False
    if ';' in api_key:
        is_cookie = True
    elif '=' in api_key and not api_key.strip().startswith('ey'):
        is_cookie = True

    if is_cookie:
        # Cookie形式
        headers = {
            'Cookie': api_key,
            'Content-Type': 'application/json'
        }
        # CSRFトークンをヘッダーに追加（Cookieから抽出）
        try:
            for part in api_key.split(';'):
                if '=' in part:
                    key, value = part.strip().split('=', 1)
                    if 'csrf_token' in key:
                        headers['X-CSRF-Token'] = value
                        break
        except Exception:
            pass
    else:
        # Bearer token形式
        headers = {
            'Authorization': f'Bearer {api_key}',
            'Content-Type': 'application/json'
        }

    response = requests.get(url, headers=headers, timeout=30)
    response.raise_for_status()

    return response.text


def sanitize_filename(name: str) -> str:
    """
    ファイル名として安全な文字列に変換
    """
    # 使えない文字を置換
    invalid_chars = '<>:"/\\|?*'
    for char in invalid_chars:
        name = name.replace(char, '-')

    # 空白をハイフンに
    name = name.replace(' ', '-')

    # 連続するハイフンを1つに
    while '--' in name:
        name = name.replace('--', '-')

    return name.strip('-').lower()


def export_workflows(dify_url: str, api_key: str, output_dir: str):
    """
    全ワークフローをエクスポート
    """
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)

    print(f"Fetching apps from {dify_url}...")
    apps = get_apps_list(dify_url, api_key)

    print(f"Found {len(apps)} apps")

    for app in apps:
        app_id = app['id']
        app_name = app['name']
        app_mode = app.get('mode', 'unknown')

        print(f"\nExporting: {app_name} (ID: {app_id}, Mode: {app_mode})")

        # ディレクトリ名を作成
        dir_name = sanitize_filename(app_name)
        app_dir = output_path / dir_name
        app_dir.mkdir(exist_ok=True)

        try:
            # DSLをエクスポート
            dsl_content = export_app_dsl(dify_url, api_key, app_id)

            # workflow.ymlに保存
            workflow_file = app_dir / 'workflow.yml'
            workflow_file.write_text(dsl_content, encoding='utf-8')
            print(f"  ✓ Saved DSL to {workflow_file}")

            # メタデータを保存
            metadata = {
                'id': app_id,
                'name': app_name,
                'mode': app_mode,
                'description': app.get('description', ''),
                'icon': app.get('icon', ''),
                'icon_background': app.get('icon_background', ''),
                'created_at': app.get('created_at', ''),
                'updated_at': app.get('updated_at', ''),
            }

            metadata_file = app_dir / 'metadata.json'
            metadata_file.write_text(
                json.dumps(metadata, indent=2, ensure_ascii=False),
                encoding='utf-8'
            )
            print(f"  ✓ Saved metadata to {metadata_file}")

        except Exception as e:
            print(f"  ✗ Error exporting {app_name}: {e}")
            continue

    print(f"\n✓ Export completed: {len(apps)} apps exported to {output_dir}")


def main():
    parser = argparse.ArgumentParser(
        description='Export Dify workflows to YAML files'
    )
    parser.add_argument(
        '--dify-url',
        required=True,
        help='Dify instance URL (e.g., https://d2iwepznz8u1l2.cloudfront.net)'
    )
    parser.add_argument(
        '--api-key',
        required=True,
        help='Dify Console API key'
    )
    parser.add_argument(
        '--output-dir',
        default='production',
        help='Output directory for exported workflows (default: production)'
    )

    args = parser.parse_args()

    try:
        export_workflows(args.dify_url, args.api_key, args.output_dir)
    except Exception as e:
        print(f"\n✗ Export failed: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == '__main__':
    main()
