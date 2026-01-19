# SpecKit v6 - GitHub spec-kit準拠の仕様策定支援ワークフロー

[GitHub spec-kit](https://github.com/github/spec-kit)の仕様策定フレームワークをDifyで実装したチャットボットです。

## 概要

プロジェクトの仕様策定をSpec-Driven Development (SDD)手法でサポートします。

- **アプリ名**: SpecKit v6
- **モード**: Chatflow (advanced-chat)
- **LLMプロバイダー**: AWS Bedrock (Amazon Nova)
- **準拠**: GitHub spec-kit (github/spec-kit)

## 利用可能なコマンド

| コマンド | 説明 | spec-kit準拠 |
|---------|------|-------------|
| `constitution` | プロジェクト原則定義（8ステップワークフロー） | constitution.md |
| `specify` | 要件仕様書作成（明確化マーカー付き） | specify.md |
| `clarify` | 曖昧要件の明確化（最大5問、9カテゴリ） | clarify.md |
| `plan` | 技術実装計画（3フェーズ: 研究→設計→報告） | plan.md |
| `tasks` | タスク分解（[TaskID][P?][Story?]形式） | tasks.md |
| `analyze` | Read-only分析（重大度分類付き） | analyze.md |
| `checklist` | 要件品質チェック（実装テストではない） | checklist.md |
| `implement` | タスク実行（TDDベース） | implement.md |
| `export` | 成果物エクスポート | - |

## 使い方

### 推奨ワークフロー

```
constitution → specify → clarify → plan → tasks → checklist → analyze → implement
```

1. **constitution**: プロジェクトの原則を定義（バージョン管理付き）
2. **specify**: 詳細な要件仕様を作成（`[NEEDS CLARIFICATION]`マーカー付き）
3. **clarify**: 曖昧な点を最大5問で明確化（9カテゴリ分類）
4. **plan**: 3フェーズで技術計画を立案（Research→Design→Planning）
5. **tasks**: `[T001] [P] [US1]`形式でタスク分解
6. **checklist**: 要件品質をチェック（実装テストではなく要件の品質）
7. **analyze**: Read-only分析で問題を検出（重大度: CRITICAL/HIGH/MEDIUM/LOW）
8. **implement**: TDDワークフローでタスク実行
9. **export**: 全成果物を出力

### 入力例

```
constitution ECサイトを構築したい
```

```
specify ユーザー認証機能
```

```
clarify
```

```
checklist security
```

## spec-kit準拠の詳細

### constitution（8ステップワークフロー）

1. テンプレート読込
2. 値収集（プロジェクト名、バージョン、日付）
3. ドラフト作成（宣言的でテスト可能な原則）
4. 一貫性伝播
5. 同期レポート（HTMLコメントで変更記録）
6. 検証
7. 出力
8. サマリー

出力例:
```markdown
# ECサイト Constitution
Version: 1.0.0
Ratified: 2026-01-19

## Core Principles
1. すべての関数は単一責任原則に従う
2. ユーザーデータは暗号化して保存する
...
```

### clarify（9カテゴリ分類、最大5問）

**カテゴリ:**
1. Functional Scope & Behavior
2. Domain & Data Model
3. Interaction & UX Flow
4. Non-Functional Quality
5. Integration & Dependencies
6. Edge Cases & Failure
7. Constraints & Tradeoffs
8. Terminology & Consistency
9. Completion Signals

**ルール:**
- 最大5問（セッション全体で）
- 1問ずつ提示
- 優先順位 = Impact × Uncertainty
- 選択式または短答式

### tasks（spec-kit形式）

```markdown
## Phase: Setup
- [ ] [T001] [P] Initialize project structure in /src
- [ ] [T002] [P] Configure build tools in /build

## Phase: User Story - US1
- [ ] [T003] [US1] Implement login form in /src/components/LoginForm.tsx
- [ ] [T004] [US1] [P] Add validation in /src/utils/validation.ts
```

### analyze（Read-only、重大度分類）

**重大度:**
| 重大度 | 基準 |
|--------|------|
| CRITICAL | ベースラインをブロック / constitution MUSTに違反 |
| HIGH | 競合 / セキュリティの曖昧さ |
| MEDIUM | 用語のドリフト |
| LOW | スタイルの問題 |

### checklist（要件品質チェック）

**禁止パターン（実装テスト）:**
- ❌ 「ボタンが正しくクリックできることを確認」

**推奨パターン（要件品質）:**
- ✓ 「エラーシナリオが要件に完全に記載されているか？」

## アーキテクチャ

```
Start
  ↓
Clarify State Checker (IF/ELSE)
  ├─ clarify進行中 → Clarify Handler (継続)
  └─ それ以外 ↓
       Command Extractor (Code Node)
         ↓
       Command Router (IF/ELSE) ← 決定論的ルーティング
         ├─ constitution → Constitution Handler → Save Constitution
         ├─ specify → Specify Handler → Save Specification
         ├─ clarify → Clarify Handler → Extract State → Save State
         ├─ plan → Plan Handler → Save Plan
         ├─ tasks → Tasks Handler → Save Tasks
         ├─ analyze → Analyze Handler → Save Analysis
         ├─ checklist → Checklist Handler → Save Checklist
         ├─ implement → Implement Handler → Save State
         ├─ export → Export Handler
         └─ (else) → Help Handler
                      ↓
               Result Aggregator
                      ↓
                   Answer
```

**特徴:**
- Question Classifierを廃止し、Code Node + IF/ELSEで決定論的ルーティング
- 各Handlerは既存の会話変数を参照し、追記形式で出力
- 複数回のspecify/plan/tasksで内容が蓄積される

## Conversation Variables

| 変数名 | 説明 |
|-------|------|
| `constitution` | プロジェクト原則（バージョン管理） |
| `specification` | 要件仕様書（spec.md形式） |
| `clarify_state` | 明確化セッション状態（質問カウント、回答履歴） |
| `implementation_plan` | 技術実装計画（plan.md形式） |
| `task_list` | タスクリスト（tasks.md形式） |
| `analysis_report` | 分析レポート |
| `checklist` | 品質チェックリスト |
| `implement_state` | 実装進捗状態 |

## バージョン履歴

| バージョン | 日付 | 変更内容 |
|-----------|------|----------|
| v6.2 | 2026-01-20 | Question ClassifierをCode Node + IF/ELSEに置換（決定論的ルーティング）、Handlerに追記機能追加 |
| v6.1 | 2026-01-19 | spec-kitテンプレート形式を完全準拠（plan-template.md, tasks-template.md, checklist-template.md の正確な形式を使用） |
| v6 | 2026-01-19 | GitHub spec-kit完全準拠（checklist, implement追加、全コマンド刷新） |
| v5 | 2026-01-18 | Variable Assigner問題を回避（Variable Aggregator使用） |
| v4 | 2026-01-17 | 22ノード完全実装版 |

## アクセス方法

1. http://yida-dify.duckdns.org/explore/apps にアクセス
2. 左サイドバーから「SpecKit v6」を選択
3. チャット欄にコマンドと内容を入力

## 参考

- [GitHub SpecKit](https://github.com/github/spec-kit) - 公式リポジトリ
- [Spec-driven development with AI](https://github.blog/ai-and-ml/generative-ai/spec-driven-development-with-ai-get-started-with-a-new-open-source-toolkit/) - GitHub公式ブログ
