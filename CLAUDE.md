# Dify Workflows リポジトリ

このリポジトリは Dify ワークフローの DSL を管理しています。

## ディレクトリ構造

```
production/           # 本番環境のワークフロー
├── {app-name}/
│   ├── workflow.yml  # DSL ファイル
│   └── metadata.json # メタデータ
templates/            # Dify 公式テンプレート（参照用）
```

## Dify DSL 生成ルール

DSL を生成・編集する際は、以下のルールを厳守すること。

### 1. ノード ID
- **必ず数字の文字列**を使用する（例: `'1734567890001'`）
- ❌ `'start'`, `'llm-node'` などの文字列 ID は不可
- ✅ `'1734567890001'`, `'1734567890002'` など

### 2. Edge（接続）の必須フィールド
```yaml
edges:
- data:
    isInIteration: false  # 必須
    isInLoop: false       # 必須
    sourceType: start
    targetType: llm
  id: 1734567890001-source-1734567890002-target
  source: '1734567890001'
  sourceHandle: source
  target: '1734567890002'
  targetHandle: target
  type: custom
  zIndex: 0               # 必須
```

### 3. ノードの必須フィールド
```yaml
nodes:
- data:
    # ... ノード固有のデータ
  height: 88
  id: '1734567890001'
  position:
    x: 80
    y: 300
  positionAbsolute:       # 必須（position と同じ値）
    x: 80
    y: 300
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 242
```

### 4. LLM ノードの prompt_template
```yaml
prompt_template:
- edition_type: basic     # 必須
  id: a1b2c3d4-e5f6-7890-abcd-ef1234567890  # UUID 形式
  role: system
  text: "プロンプト内容"
- id: b2c3d4e5-f6a7-8901-bcde-f23456789012
  role: user
  text: "{{#1734567890001.query#}}"
```

### 4.1 LLM モデル設定（AWS Bedrock / Amazon Nova）
```yaml
model:
  completion_params:
    temperature: 0.7
  mode: chat
  name: amazon nova
  provider: langgenius/bedrock/bedrock
```

### 4.2 依存関係（AWS Bedrock）
```yaml
dependencies:
- current_identifier: null
  type: marketplace
  value:
    marketplace_plugin_unique_identifier: langgenius/bedrock:0.0.49@8bca05c0cfdbc60cc824b18410dea65ad6e1303099bcaa768a9de20971e3eaf4
    version: null
```

### 5. LLM ノードの追加必須フィールド
```yaml
structured_output_enabled: false  # 必須
vision:
  enabled: false                  # 必須
```

### 6. 変数参照の形式
- `{{#ノードID.変数名#}}` の形式を使用
- 例: `{{#1734567890001.query#}}`

### 7. 参照すべきテンプレート
新しい DSL を生成する前に、`templates/` ディレクトリ内の公式テンプレートを参照すること。
特に使用するノードタイプに近いテンプレートを確認する。
