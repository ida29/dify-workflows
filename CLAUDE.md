# Dify Workflows リポジトリ（セルフホスト版）

このリポジトリは AWS EC2 上の Dify セルフホスト環境のワークフロー DSL を管理しています。

## 環境情報

- **Dify URL**: http://13.230.151.56
- **Dify Version**: 1.10.0
- **LLM Provider**: AWS Bedrock (Amazon Nova)
- **インフラ**: AWS CDK (EC2 Spot + Elastic IP)

## ディレクトリ構造

```
.claude/
└── skills/
    └── dify-dsl.md        # DSL生成スキル（詳細なテンプレート）
.github/
└── workflows/
    └── export-daily.yml   # 毎日3AM JSTに自動バックアップ
workflows/                 # ワークフロー DSL
├── {app-name}/
│   ├── workflow.yml       # DSL ファイル
│   └── metadata.json      # メタデータ
scripts/
├── export-workflows.py    # ワークフローエクスポート
├── get-access-token.py    # Difyログイン・トークン取得
└── requirements.txt
infra/                     # AWS CDK インフラ定義
├── lib/
│   └── dify-stack.ts      # EC2 Spot + Bedrock
└── scripts/
    └── user-data.sh       # EC2初期化スクリプト
```

## 現在のワークフロー

| 名前 | モード | 説明 |
|------|--------|------|
| 日本語→英語翻訳 | workflow | シンプルな翻訳 |
| コード変換機 | completion | プログラミング言語変換 |
| 顧客レビュー分析 | workflow | 質問分類器でルーティング |
| 質問分類器 + 知識 + チャットボット | advanced-chat | RAG対応チャット |
| ウェブの検索と要約 | workflow | HTTP Request連携 |
| 人気科学文章の著者 | advanced-chat | ネストされた並列処理 |
| DeepResearch | advanced-chat | 深掘りリサーチ |

## DSL 生成

DSL生成の詳細ルールは `.claude/skills/dify-dsl.md` を参照。

### クイックリファレンス

#### ノード ID
- **必ず数字の文字列**（タイムスタンプ形式）を使用
- ✅ `'1735638000001'`
- ❌ `'start'`, `'llm-node'`

#### モデル設定（AWS Bedrock）
```yaml
model:
  completion_params:
    temperature: 0.7
  mode: chat
  name: us.amazon.nova-lite-v1:0
  provider: langgenius/bedrock/bedrock
```

#### 依存関係
```yaml
dependencies:
- current_identifier: null
  type: marketplace
  value:
    marketplace_plugin_unique_identifier: langgenius/bedrock:0.0.49@8bca05c0cfdbc60cc824b18410dea65ad6e1303099bcaa768a9de20971e3eaf4
    version: null
```

#### 変数参照
```
{{#ノードID.変数名#}}
```
例: `{{#1735638000001.input#}}`

## 運用コマンド

### ワークフローのバックアップ（手動）
```bash
# ログインしてトークン取得
python scripts/get-access-token.py \
  --dify-url "http://13.230.151.56" \
  --email "YOUR_EMAIL" \
  --password "YOUR_PASSWORD" \
  --json > /tmp/dify_login.json

# エクスポート
COOKIES=$(jq -r '.cookies' /tmp/dify_login.json)
python scripts/export-workflows.py \
  --dify-url "http://13.230.151.56" \
  --api-key "$COOKIES" \
  --output-dir workflows/
```

### インフラ操作
```bash
cd infra

# デプロイ
npx cdk deploy --require-approval never

# EC2接続（SSM）
aws ssm start-session --target <instance-id>

# Difyコンテナ確認
cd /opt/dify/docker && docker compose ps
```
