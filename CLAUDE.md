# Dify Workflows リポジトリ（セルフホスト版）

このリポジトリは AWS EC2 上の Dify セルフホスト環境のワークフロー DSL を管理しています。

## 環境情報

- **Dify URL**: http://yida-dify.duckdns.org
- **Dify Version**: 1.10.0
- **LLM Provider**: AWS Bedrock (Amazon Nova)
- **インフラ**: AWS CDK (EC2 On-Demand + Elastic IP + DuckDNS)

## インストール済みプラグイン

| プラグイン | 用途 |
|-----------|------|
| langgenius/bedrock | LLMプロバイダー（Amazon Nova） |
| langgenius/tavily | Web検索・コンテンツ抽出 |

**注意**: OpenAI、DeepSeek、Jina AIプラグインは削除済み。全てのワークフローはBedrock + Tavilyで動作する。

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
| ウェブの検索と要約 | workflow | Tavily検索・抽出連携 |
| 人気科学文章の著者 | advanced-chat | ネストされた並列処理 |
| DeepResearch | advanced-chat | 深掘りリサーチ |

## DSL 生成

**DSL生成時は `.claude/skills/dify-dsl.md` を必ず参照すること。**

スキルには以下が含まれている:
- ノードタイプ別テンプレート（Start, LLM, End, Answer, Question Classifier等）
- Edge（接続）テンプレート
- 変数参照形式
- AWS Bedrock設定
- 生成時の注意事項

### モデル命名規則

DifyのDSLでは、モデル名にフレンドリーネームを使用する:

```yaml
# 正しい（フレンドリーネーム）
model:
  name: amazon nova
  provider: langgenius/bedrock/bedrock

# 間違い（Bedrock モデルID）
model:
  name: amazon.nova-pro-v1:0  # エラーになる
```

### Tavily ツール設定

```yaml
# Web検索
provider_id: langgenius/tavily/tavily
tool_name: tavily_search

# コンテンツ抽出
provider_id: langgenius/tavily/tavily
tool_name: tavily_extract
tool_parameters:
  urls:
    type: mixed
    value: '{{#node_id.item#}}'
# 出力は .raw_content で参照
```

### Knowledge Retrieval 設定

**重要**: Knowledge Retrievalノードの「Retrieval Setting」では、「Rerank Model」ではなく「Weighted Score」を使用すること。

```yaml
# ⚠️ Rerank Model は OpenAI プロバイダーを参照してエラーになる可能性あり
# ✅ Weighted Score を使用（推奨）
retrieval_setting:
  mode: weighted_score
  semantic: 1.0
  keyword: 0
  top_k: 4
```

**理由**: Difyの Knowledge Retrieval ノードで「Rerank Model」を選択すると、内部的にOpenAIのrerankモデルを参照しようとし、OpenAIプラグインがインストールされていない環境では「Provider openai does not exist」エラーが発生する。

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
