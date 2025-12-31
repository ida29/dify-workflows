# Dify Workflows (個人用 / VibeCoding)

Dify セルフホスト版のワークフロー DSL バックアップ用リポジトリ。

## リポジトリ構造

```
.
├── workflows/           # ワークフロー DSL
│   ├── app-name-1/
│   │   ├── workflow.yml
│   │   └── metadata.json
│   └── app-name-2/
│       ├── workflow.yml
│       └── metadata.json
├── scripts/             # 自動化スクリプト
│   ├── export-workflows.py
│   ├── get-access-token.py
│   └── requirements.txt
├── infra/               # AWS CDK (Dify インフラ)
├── CLAUDE.md            # Claude Code 用 DSL ルール
└── README.md
```

## セットアップ

```bash
pip install -r scripts/requirements.txt
```

## 使い方

### 手動エクスポート

```bash
# トークン取得
python scripts/get-access-token.py \
  --dify-url http://13.230.151.56 \
  --email y.ida898@gmail.com \
  --password YOUR_PASSWORD

# エクスポート
python scripts/export-workflows.py \
  --dify-url http://13.230.151.56 \
  --api-key YOUR_ACCESS_TOKEN \
  --output-dir workflows/
```

## 自動バックアップ (GitHub Actions)

毎日 3:00 JST に自動でワークフローをエクスポートし、変更があればコミットします。

### GitHub Secrets の設定

1. リポジトリの Settings → Secrets and variables → Actions
2. 以下の Secrets を追加:

| Name | Value |
|------|-------|
| `DIFY_URL` | `http://13.230.151.56` |
| `DIFY_EMAIL` | Dify ログイン用メールアドレス |
| `DIFY_PASSWORD` | Dify ログイン用パスワード |

### 手動実行

Actions タブ → "Export Workflows Daily" → "Run workflow" で手動実行も可能です。

## インフラ

Dify は AWS 東京リージョンにスポットインスタンスでホスティングしています。

詳細: [infra/README.md](infra/README.md)

## 環境

- **Dify セルフホスト**: http://13.230.151.56
- **AWS アカウント**: 610424051919
- **リージョン**: ap-northeast-1 (東京)
