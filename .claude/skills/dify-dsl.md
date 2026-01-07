# Dify DSL Generator Skill

このスキルはDify Workflow DSLを生成するための専門知識を提供します。

## DSL生成の基本ルール

### 1. ノードID規則
- **必ず数字の文字列（タイムスタンプ形式）を使用**
- 例: `'1734567890001'`, `'1734567890002'`
- 文字列ID（`'start'`, `'llm-node'`等）は使用不可

### 2. アプリモード
Difyには以下のアプリモードがある:
- `completion`: テキスト生成（シンプルな入力→出力）
- `workflow`: ワークフロー（複数ノードの連携、startノードとendノードが必要）
- `advanced-chat`: チャットフロー（会話型、answerノードで応答）

### 3. 基本構造

```yaml
app:
  description: ''
  icon: 🤖
  icon_background: '#FFEAD5'
  mode: workflow  # completion | workflow | advanced-chat
  name: アプリ名
  use_icon_as_answer_icon: false
dependencies:
- current_identifier: null
  type: marketplace
  value:
    marketplace_plugin_unique_identifier: langgenius/bedrock:0.0.49@8bca05c0cfdbc60cc824b18410dea65ad6e1303099bcaa768a9de20971e3eaf4
    version: null
kind: app
version: 0.5.0
workflow:
  conversation_variables: []
  environment_variables: []
  features:
    file_upload:
      image:
        enabled: false
        number_limits: 3
        transfer_methods:
        - local_file
        - remote_url
    opening_statement: ''
    retriever_resource:
      enabled: false
    sensitive_word_avoidance:
      enabled: false
    speech_to_text:
      enabled: false
    suggested_questions: []
    suggested_questions_after_answer:
      enabled: false
    text_to_speech:
      enabled: false
      language: ''
      voice: ''
  graph:
    edges: []
    nodes: []
    viewport:
      x: 0
      y: 0
      zoom: 1
  rag_pipeline_variables: []
```

## ノードタイプ別テンプレート

### Start ノード（開始）
```yaml
- data:
    desc: ''
    selected: false
    title: Start
    type: start
    variables:
    - label: Input
      max_length: 2000
      options: []
      required: true
      type: paragraph  # text-input | paragraph | select | number
      variable: input
  height: 89
  id: '1734567890001'
  position:
    x: 80
    y: 300
  positionAbsolute:
    x: 80
    y: 300
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 244
```

### LLM ノード
```yaml
- data:
    context:
      enabled: false
      variable_selector: []
    desc: ''
    memory:
      role_prefix:
        assistant: ''
        user: ''
      window:
        enabled: false
        size: 50
    model:
      completion_params:
        temperature: 0.7
      mode: chat
      name: amazon nova
      provider: langgenius/bedrock/bedrock
    prompt_template:
    - edition_type: basic
      id: a1b2c3d4-e5f6-7890-abcd-ef1234567890
      role: system
      text: 'あなたは優秀なアシスタントです。'
    - id: b2c3d4e5-f6a7-8901-bcde-f23456789012
      role: user
      text: '{{#1734567890001.input#}}'
    selected: false
    structured_output_enabled: false
    title: LLM
    type: llm
    variables: []
    vision:
      enabled: false
  height: 98
  id: '1734567890002'
  position:
    x: 380
    y: 300
  positionAbsolute:
    x: 380
    y: 300
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 244
```

### End ノード（終了）- workflow モード用
```yaml
- data:
    desc: ''
    outputs:
    - value_selector:
      - '1734567890002'
      - text
      variable: output
    selected: false
    title: End
    type: end
  height: 90
  id: '1734567890003'
  position:
    x: 680
    y: 300
  positionAbsolute:
    x: 680
    y: 300
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 244
```

### Answer ノード（応答）- advanced-chat モード用
```yaml
- data:
    answer: '{{#1734567890002.text#}}'
    desc: ''
    selected: false
    title: Answer
    type: answer
    variables:
    - value_selector:
      - '1734567890002'
      - text
      variable: text
  height: 103
  id: '1734567890003'
  position:
    x: 680
    y: 300
  positionAbsolute:
    x: 680
    y: 300
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 244
```

### Question Classifier ノード（質問分類器）
```yaml
- data:
    classes:
    - id: '1734567890101'
      name: カテゴリA
    - id: '1734567890102'
      name: カテゴリB
    desc: ''
    instructions: ''
    model:
      completion_params:
        temperature: 0.7
      mode: chat
      name: amazon nova
      provider: langgenius/bedrock/bedrock
    query_variable_selector:
    - '1734567890001'
    - input
    selected: false
    title: Question Classifier
    topics: []
    type: question-classifier
  height: 183
  id: '1734567890010'
  position:
    x: 380
    y: 300
  positionAbsolute:
    x: 380
    y: 300
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 244
```

### Knowledge Retrieval ノード（知識検索）
```yaml
- data:
    dataset_ids: []  # 知識ベースIDを設定
    desc: ''
    query_variable_selector:
    - '1734567890001'
    - input
    retrieval_mode: single
    selected: false
    single_retrieval_config:
      model:
        completion_params: {}
        mode: chat
        name: amazon nova
        provider: langgenius/bedrock/bedrock
    title: Knowledge Retrieval
    type: knowledge-retrieval
  height: 96
  id: '1734567890020'
  position:
    x: 380
    y: 300
  positionAbsolute:
    x: 380
    y: 300
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 244
```

**⚠️ Retrieval Setting の注意点**:
- Knowledge Retrieval ノードの「Retrieval Setting」では **「Weighted Score」を使用すること**
- 「Rerank Model」を選択すると、OpenAI プロバイダーを参照してエラーになる可能性がある
- UI上で設定する場合:
  - Retrieval Setting → Rerank Setting → **Weighted Score** を選択
  - Semantic: 1.0, Keyword: 0, Top K: 4 （推奨値）

### HTTP Request ノード
```yaml
- data:
    authorization:
      config: null
      type: no-auth
    body:
      data: ''
      type: none  # none | form-data | x-www-form-urlencoded | raw-text | json
    desc: ''
    headers: ''
    method: get  # get | post | put | delete | patch
    params: ''
    selected: false
    timeout:
      connect: 10
      max_connect_timeout: 300
      max_read_timeout: 600
      max_write_timeout: 600
      read: 60
      write: 20
    title: HTTP Request
    type: http-request
    url: https://api.example.com
    variables: []
  height: 155
  id: '1734567890030'
  position:
    x: 380
    y: 300
  positionAbsolute:
    x: 380
    y: 300
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 244
```

### Code ノード（コード実行）
```yaml
- data:
    code: |
      def main(arg1: str) -> dict:
          return {"result": arg1.upper()}
    code_language: python3
    desc: ''
    outputs:
      result:
        children: null
        type: string
    selected: false
    title: Code
    type: code
    variables:
    - value_selector:
      - '1734567890001'
      - input
      variable: arg1
  height: 102
  id: '1734567890040'
  position:
    x: 380
    y: 300
  positionAbsolute:
    x: 380
    y: 300
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 244
```

### IF/ELSE ノード（条件分岐）
```yaml
- data:
    cases:
    - case_id: 'true'
      conditions:
      - comparison_operator: contains
        id: cond-001
        value: キーワード
        varType: string
        variable_selector:
        - '1734567890001'
        - input
      logical_operator: and
    desc: ''
    selected: false
    title: IF/ELSE
    type: if-else
  height: 126
  id: '1734567890050'
  position:
    x: 380
    y: 300
  positionAbsolute:
    x: 380
    y: 300
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 244
```

### Variable Assigner ノード（変数集約）
```yaml
- data:
    desc: ''
    output_type: string
    selected: false
    title: Variable Assigner
    type: variable-assigner
    variables:
    - - '1734567890002'
      - text
    - - '1734567890003'
      - text
  height: 164
  id: '1734567890060'
  position:
    x: 680
    y: 300
  positionAbsolute:
    x: 680
    y: 300
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 244
```

## Edge（接続）テンプレート

### 基本接続
```yaml
- data:
    sourceType: start
    targetType: llm
  id: 1734567890001-1734567890002
  source: '1734567890001'
  sourceHandle: source
  target: '1734567890002'
  targetHandle: target
  type: custom
```

### Question Classifier からの分岐接続
```yaml
- data:
    sourceType: question-classifier
    targetType: llm
  id: 1734567890010-1734567890002
  source: '1734567890010'
  sourceHandle: '1734567890101'  # classのidを指定
  target: '1734567890002'
  targetHandle: target
  type: custom
```

### IF/ELSE からの分岐接続
```yaml
# true の場合
- data:
    sourceType: if-else
    targetType: llm
  id: 1734567890050-true-1734567890002
  source: '1734567890050'
  sourceHandle: 'true'
  target: '1734567890002'
  targetHandle: target
  type: custom

# false の場合
- data:
    sourceType: if-else
    targetType: answer
  id: 1734567890050-false-1734567890003
  source: '1734567890050'
  sourceHandle: 'false'
  target: '1734567890003'
  targetHandle: target
  type: custom
```

## 変数参照形式

変数を参照する際は以下の形式を使用:
- `{{#ノードID.変数名#}}`
- 例: `{{#1734567890001.input#}}`
- LLM出力: `{{#1734567890002.text#}}`
- Knowledge Retrieval結果: `{{#1734567890020.result#}}`

## AWS Bedrock 用 dependencies

```yaml
dependencies:
- current_identifier: null
  type: marketplace
  value:
    marketplace_plugin_unique_identifier: langgenius/bedrock:0.0.57@b7fb0414c4e64004a36c3141cb8d9a249d949013bb21efbce34918e71cce5051
    version: null
```

## Tavily ツールノード

### Tavily Search（Web検索）
```yaml
- data:
    desc: ''
    provider_id: langgenius/tavily/tavily
    provider_name: langgenius/tavily/tavily
    provider_type: api
    selected: false
    title: TavilySearch
    tool_configurations:
      exclude_domains: null
      include_answer: 0
      include_domains: null
      include_images: 0
      include_raw_content: 0
      max_results: 10
      search_depth: basic
    tool_label: TavilySearch
    tool_name: tavily_search
    tool_parameters:
      query:
        type: mixed
        value: '{{#1734567890001.input#}}'
    type: tool
  height: 245
  id: '1734567890070'
  position:
    x: 380
    y: 300
  positionAbsolute:
    x: 380
    y: 300
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 244
```

### Tavily Extract（コンテンツ抽出）
```yaml
- data:
    desc: ''
    provider_id: langgenius/tavily/tavily
    provider_name: langgenius/tavily/tavily
    provider_type: api
    selected: false
    title: Tavily Extract
    tool_configurations: {}
    tool_label: Tavily Extract
    tool_name: tavily_extract
    tool_parameters:
      urls:
        type: mixed
        value: '{{#1716911333343.item#}}'
    type: tool
  height: 150
  id: '1734567890080'
  position:
    x: 380
    y: 300
  positionAbsolute:
    x: 380
    y: 300
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 244
```

**注意**: Tavily Extractの出力は `.raw_content` で参照する:
- `{{#1734567890080.raw_content#}}`

### Tavily用 dependencies
```yaml
- current_identifier: null
  type: marketplace
  value:
    marketplace_plugin_unique_identifier: langgenius/tavily:0.1.2@aa7a8744b2ccf3a7aec818da6c504997a6319b29040e541bfc73b4fbaa9e98d9
    version: null
```

## DSL生成時の注意事項

1. **ノードIDは一意のタイムスタンプ形式**で生成すること
2. **positionAbsoluteはpositionと同じ値**を設定すること
3. **edgeのidは `{source}-{target}` 形式**で生成すること
4. **workflowモードではEndノード、advanced-chatモードではAnswerノード**を使用すること
5. **変数参照は`{{#nodeId.varName#}}`形式**を厳守すること
6. **LLMノードのprompt_templateにはedition_type: basicを必ず含める**こと
7. **既存のテンプレートワークフローを参照**して、使用するノードタイプの構造を確認すること

## ⚠️ 分岐ワークフローの重要ルール（必読）

### Difyの変数参照の重大な制約

**🚨 重要**: Difyでは、**実行されなかったノードの変数は参照できない**。これはDifyの仕様であり、回避不可能。

### NGパターン1: 分岐後に複数パスの変数を参照

```
Start
  ↓
IF/ELSE
  ├─ 有り → LLM-A(#021) ───┐
  └─ 無し → LLM-B(#005) ───┤
                           ↓
                      LLM-C（エラー！）
                      {{#021.text#}}  ← 無しパスでは存在しない
                      {{#005.text#}}  ← 有りパスでは存在しない
```

**結果**: どちらのパスでも必ずエラー。「実行されなかったノードの変数は空になる」という動作ではなく、**Variable not found エラー**になる。

### NGパターン2: 複数入力エッジの共有ノード

```
Start
  ↓
IF/ELSE
  ├─ 有り → LLM-A(#021) ───┐
  │                        ├──→ 共有LLM-C（エラー！）
  └─ 無し → LLM-B(#005) ───┘     {{#021.text#}} or {{#005.text#}}
```

**結果**: 共有ノードが複数パスの変数を参照しようとすると、実行されなかったパスの変数でエラー。

### ✅ 正しい解決策: Variable Aggregator で合流

**Variable Aggregator**（変数集約）を使って、分岐した複数パスを**1つの出力変数に統合**してから後続処理へ渡す。

```
Start
  ↓
IF/ELSE
  ├─ 有り → LLM-A(#021) ───────────┐
  └─ 無し → API → IF/ELSE         │
                   ├─ 有り → LLM-B(#005) ─┤
                   └─ 無し → LLM-C(#020) ─┤
                                          ↓
                          Variable Aggregator(#030)
                          （3つのパスを1つに統合）
                                          ↓
                                    後続LLM
                                    {{#030.output#}}  ← これなら確実に存在！
                                          ↓
                                        ...続く
```

### Variable Aggregator ノードテンプレート

```yaml
# Variable Aggregator（分岐合流用）
- data:
    advanced_settings:
      group_enabled: false
    desc: '複数パスの出力を1つに統合'
    output_type: string
    selected: false
    title: パス合流
    type: variable-aggregator
    variables:
    - - '1735660001021'  # パス1のノード
      - text
    - - '1735660001005'  # パス2のノード
      - text
    - - '1735660001020'  # パス3のノード
      - text
  height: 150
  id: '1735660001030'
  position:
    x: 1880
    y: 400
  positionAbsolute:
    x: 1880
    y: 400
  selected: false
  sourcePosition: right
  targetPosition: left
  type: custom
  width: 244
```

### Variable Aggregator への Edge テンプレート

```yaml
# 各パスから Variable Aggregator への接続
- data:
    isInIteration: false
    sourceType: llm
    targetType: variable-aggregator
  id: '1735660001021-1735660001030'
  source: '1735660001021'
  sourceHandle: source
  target: '1735660001030'
  targetHandle: '1735660001021'  # sourceのノードIDを指定
  type: custom
  zIndex: 0
```

### 分岐設計のルール（改訂版）

1. **分岐後は必ず Variable Aggregator で合流**: IF/ELSE で分岐したパスは、後続処理の前に Variable Aggregator で統合する
2. **後続ノードは Aggregator の出力のみ参照**: `{{#aggregatorNodeId.output#}}` を使用し、個別パスのノードは参照しない
3. **並列処理は分岐合流後に配置**: 分岐がある場合、並列処理は Variable Aggregator の後に配置する
4. **変数参照の検証**: 各ノードで参照する変数が、**そのノードに至る全てのパスで確実に存在する**ことを確認

### Variable Assigner vs Variable Aggregator

| 用途 | ノードタイプ | 説明 |
|------|-------------|------|
| **分岐パスの合流** | `variable-aggregator` | IF/ELSE等の分岐を1つに統合 |
| **並列処理の集約** | `variable-assigner` | 同時実行ノードの結果を収集 |

**注意**: 名前が似ているが用途が異なる。分岐合流には必ず `variable-aggregator` を使用。

## 参照すべきテンプレート

`workflows/`ディレクトリ内の既存ワークフローを参照:
- `コード変換機/` - completionモードの例
- `顧客レビュー分析/` - workflowモード + Question Classifier
- `質問分類器-+-知識-+-チャットボット/` - advanced-chatモード + Knowledge Retrieval
- `ウェブの検索と要約のワークフローパターン/` - Tavily検索・抽出 + Iteration
- `人気科学文章の著者-(ネストされた並列)/` - 並列処理の例
- `ts-youtube-content-generator/` - **IF/ELSE分岐 + Variable Aggregator による合流**（分岐設計の参考に）
