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
    marketplace_plugin_unique_identifier: langgenius/bedrock:0.0.49@8bca05c0cfdbc60cc824b18410dea65ad6e1303099bcaa768a9de20971e3eaf4
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

## 参照すべきテンプレート

`workflows/`ディレクトリ内の既存ワークフローを参照:
- `コード変換機/` - completionモードの例
- `顧客レビュー分析/` - workflowモード + Question Classifier
- `質問分類器-+-知識-+-チャットボット/` - advanced-chatモード + Knowledge Retrieval
- `ウェブの検索と要約のワークフローパターン/` - HTTP Request
- `人気科学文章の著者-(ネストされた並列)/` - 並列処理の例
