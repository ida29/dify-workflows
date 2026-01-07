#!/usr/bin/env python3
"""
Dify Access Token取得スクリプト (セルフホスト版)

Difyにログインしてアクセストークンを取得します。
Dify 1.10.x ではパスワードをBase64エンコードして送信します。
"""
import sys
import argparse
import base64
import requests
import json


def encode_password(password: str) -> str:
    """
    パスワードをBase64エンコード
    DifyのencryptPassword関数と同等の処理
    """
    return base64.b64encode(password.encode('utf-8')).decode('utf-8')


def get_access_token(dify_url: str, email: str, password: str) -> dict:
    """
    Difyにログインしてアクセストークンを取得
    Cookieとして返す（access_token + csrf_token）
    """
    url = f"{dify_url}/console/api/login"
    headers = {
        'Content-Type': 'application/json'
    }
    # Dify 1.10.x ではパスワードをBase64エンコードして送信
    payload = {
        'email': email,
        'password': encode_password(password)
    }

    response = requests.post(url, headers=headers, json=payload, timeout=30)
    response.raise_for_status()

    # クッキーからトークンを取得
    access_token = None
    csrf_token = None

    for cookie in response.cookies:
        if cookie.name in ('__Host-access_token', 'access_token'):
            access_token = cookie.value
        elif cookie.name in ('__Host-csrf_token', 'csrf_token'):
            csrf_token = cookie.value

    if not access_token:
        raise ValueError(f"Access token not found. Cookies: {response.cookies}")

    # Cookie文字列を構築
    cookie_string = f"access_token={access_token}"
    if csrf_token:
        cookie_string += f"; csrf_token={csrf_token}"

    return {
        'access_token': access_token,
        'csrf_token': csrf_token or '',
        'cookies': cookie_string,
        'result': 'success'
    }


def main():
    parser = argparse.ArgumentParser(
        description='Get Dify access token (self-hosted)'
    )
    parser.add_argument(
        '--dify-url',
        required=True,
        help='Dify instance URL (e.g., http://13.230.151.56)'
    )
    parser.add_argument(
        '--email',
        required=True,
        help='Your Dify login email'
    )
    parser.add_argument(
        '--password',
        required=True,
        help='Your Dify login password'
    )
    parser.add_argument(
        '--json',
        action='store_true',
        help='Output in JSON format (for automation)'
    )

    args = parser.parse_args()

    try:
        if not args.json:
            print(f"Logging in to {args.dify_url}...", file=sys.stderr)

        result = get_access_token(args.dify_url, args.email, args.password)

        if args.json:
            print(json.dumps(result))
        else:
            print("\nLogin successful!", file=sys.stderr)
            print("\nCookies (use this for export-workflows.py --api-key):")
            print(result['cookies'])

    except requests.exceptions.HTTPError as e:
        print(f"\nLogin failed: {e}", file=sys.stderr)
        if e.response is not None:
            print(f"Response: {e.response.text}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"\nError: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == '__main__':
    main()
