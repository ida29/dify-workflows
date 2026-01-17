# SpecKit Complete - 仕様策定支援ワークフロー

GitHub SpecKitの仕様策定フレームワークをDifyで実装したチャットボットです。

## 概要

プロジェクトの仕様策定を段階的にサポートします。

- **アプリ名**: SpecKit Complete Final 30
- **モード**: Chatflow (advanced-chat)
- **LLMプロバイダー**: AWS Bedrock (Amazon Nova)

## 利用可能なコマンド

| コマンド | 説明 | 用途 |
|---------|------|------|
| `constitution` | プロジェクト原則定義 | プロジェクトの基本方針・ガイドラインを定義 |
| `specify` | 要件仕様書作成 | 機能要件・非機能要件を整理 |
| `clarify` | 曖昧な点の明確化 | 不明確な要件を質問形式で明確化 |
| `plan` | 技術実装計画 | アーキテクチャ・技術スタック選定 |
| `tasks` | タスク分解 | 実装タスクを優先順位付きで分解 |
| `analyze` | 要件分析 | 一貫性チェック・リスク分析 |
| `review` | 全体レビュー | プロジェクト全体の評価・改善提案 |
| `export` | 成果物エクスポート | 全成果物をMarkdown形式で出力 |

## 使い方

### 基本的な流れ

1. **constitution**: まずプロジェクトの原則を定義
2. **specify**: 詳細な要件仕様を作成
3. **clarify**: 曖昧な点を対話形式で明確化
4. **plan**: 技術的な実装計画を立案
5. **tasks**: 具体的なタスクに分解
6. **review**: 全体をレビュー
7. **export**: 成果物を出力

### 入力例

```
constitution ECサイトを構築したい
```

```
specify ユーザー認証機能の要件を整理して
```

```
tasks API開発のタスクを分解して
```

## アーキテクチャ

```
Start
  ↓
Command Validator (IF/ELSE)
  ├─ 有効なコマンド → Command Parser → Question Classifier
  │                                        ├─ constitution → Constitution Handler
  │                                        ├─ specify → Specify Handler → Clarification Detector
  │                                        ├─ clarify → Clarify Handler
  │                                        ├─ plan → Plan Handler
  │                                        ├─ tasks → Tasks Handler
  │                                        ├─ analyze → Analyze Handler
  │                                        ├─ review → Review Handler
  │                                        └─ export → Export Handler
  └─ 無効なコマンド → Error Handler
                           ↓
                    Result Aggregator
                           ↓
                    Format Converter
                           ↓
                    Memory Updater
                           ↓
                        Answer
```

## Conversation Variables

セッション間で状態を保持するための変数：

| 変数名 | 説明 |
|-------|------|
| `constitution` | プロジェクト原則 |
| `specification` | 要件仕様書 |
| `clarified_requirements` | 明確化された要件 |
| `implementation_plan` | 技術実装計画 |
| `task_list` | タスクリスト |
| `analysis_result` | 分析結果 |
| `current_command` | 現在のコマンド |
| `clarification_items` | 明確化が必要な項目 |
| `clarification_progress` | 明確化の進捗 |

## アクセス方法

1. http://yida-dify.duckdns.org/explore/apps にアクセス
2. 左サイドバーから「SpecKit Complete Final 30」を選択
3. チャット欄にコマンドと内容を入力

## ワークフローファイル

| ファイル | 説明 |
|---------|------|
| `workflow_v1_minimal.yml` | 最小構成（3ノード） |
| `workflow_v2_classifier.yml` | Question Classifier使用版 |
| `workflow_v3_complete.yml` | 状態管理付き完全版 |
| `workflow_v4_full.yml` | 22ノード完全実装版 |

## ガイド付き対話（実装予定）

### Opening Statement

会話開始時に以下のメニューを表示：

```
🚀 SpecKit へようこそ！

プロジェクトの仕様策定をお手伝いします。
何をしたいですか？番号または内容を教えてください：

1️⃣ 新しいプロジェクトを始める（原則定義）
2️⃣ 要件・仕様を整理する
3️⃣ 曖昧な点を明確にする
4️⃣ 技術的な実装計画を立てる
5️⃣ タスクに分解する
6️⃣ 全体をレビューする
7️⃣ 成果物をエクスポートする

例: 「1」または「ECサイトを作りたい」
```

### 番号とコマンドのマッピング

| 番号 | コマンド | 説明 |
|------|----------|------|
| 1 | constitution | 新しいプロジェクトを始める（原則定義） |
| 2 | specify | 要件・仕様を整理する |
| 3 | clarify | 曖昧な点を明確にする |
| 4 | plan | 技術的な実装計画を立てる |
| 5 | tasks | タスクに分解する |
| 6 | review | 全体をレビューする |
| 7 | export | 成果物をエクスポートする |

### 入力パターン

ユーザーは以下の形式で入力可能：

1. **番号のみ**: `1` → constitution（追加入力を促す）
2. **番号 + 内容**: `1 ECサイトを作りたい` → constitution ECサイトを作りたい
3. **コマンド + 内容**: `constitution ECサイトを作りたい`（従来形式）
4. **自然言語**: `ECサイトを作りたい` → constitutionと判断

### 実装箇所

1. **Conversation Opener**: Opening Statementを有効化してメニュー表示
2. **Command Parser**: 番号入力と自然言語を解析してコマンドに変換
3. **Question Classifier**: 柔軟なルーティング

## 今後の改善予定

- [x] ガイド付き対話の設計（番号選択式メニュー）
- [ ] ガイド付き対話の実装
- [ ] 日本語応答の強化
- [ ] `implement`コマンドの追加（公式SpecKit準拠）
- [ ] `checklist`コマンドの追加

## 参考

- [GitHub SpecKit](https://github.com/github/spec-kit) - 公式リポジトリ
