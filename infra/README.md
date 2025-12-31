# Dify on AWS (Spot Instance)

低コストで Dify をセルフホストするための CDK プロジェクト。

## アーキテクチャ

```
┌─────────────────────────────────────────────┐
│  VPC (単一AZ)                                │
│  ┌─────────────────────────────────────────┐│
│  │  Public Subnet                          ││
│  │  ┌─────────────────────────────────────┐││
│  │  │  EC2 Spot Instance (t3.medium)      │││
│  │  │  ├── Docker Compose                 │││
│  │  │  │   ├── dify-web                   │││
│  │  │  │   ├── dify-api                   │││
│  │  │  │   ├── dify-worker                │││
│  │  │  │   ├── postgres                   │││
│  │  │  │   ├── redis                      │││
│  │  │  │   └── nginx                      │││
│  │  │  └── EBS 30GB (gp3, encrypted)      │││
│  │  └─────────────────────────────────────┘││
│  └─────────────────────────────────────────┘│
│  + Elastic IP (固定IP)                       │
└─────────────────────────────────────────────┘
```

## コスト (東京リージョン概算)

| リソース | 月額 |
|---------|------|
| EC2 Spot (t3.medium) | ~$12-15 |
| EBS 30GB (gp3) | ~$3 |
| Elastic IP | $0 (使用中) |
| **合計** | **~$15-18/月** |

## 前提条件

- AWS CLI が設定済み
- Node.js 18+ がインストール済み
- CDK がインストール済み (`npm install -g aws-cdk`)

## デプロイ手順

### 1. 依存関係のインストール

```bash
cd infra
npm install
```

### 2. CDK Bootstrap (初回のみ)

```bash
cdk bootstrap
```

### 3. デプロイ

```bash
# 自分のIPアドレスのみアクセスを許可する場合
MY_IP=$(curl -s https://checkip.amazonaws.com)/32
cdk deploy --context allowedIp=$MY_IP

# または全てのIPからアクセスを許可する場合 (非推奨)
cdk deploy
```

### 4. Dify にアクセス

デプロイ完了後、出力された URL にアクセス:

```
http://<Elastic IP>
```

初回アクセス時に管理者アカウントを設定します。

## SSH 接続

```bash
ssh -i <your-key.pem> ec2-user@<Elastic IP>
```

## ログ確認

```bash
# User data スクリプトのログ
cat /var/log/user-data.log

# Docker コンテナのログ
cd /opt/dify/docker
docker-compose logs -f
```

## スポットインスタンスについて

- **メリット**: オンデマンドより 60-70% 安い
- **デメリット**: AWS の需要により中断される可能性がある

### 中断対策

1. **EBS は削除されない**: `deleteOnTermination: false` で設定済み
2. **自動停止/再開**: `interruptionBehavior: STOP` で中断時は停止（終了ではない）
3. **Elastic IP**: 固定IPなので再開後も同じIPでアクセス可能

## 削除

```bash
cdk destroy
```

**注意**: EBS ボリュームは `deleteOnTermination: false` なので手動で削除が必要です。

## Bedrock 連携

インスタンスには `AmazonBedrockFullAccess` ポリシーが付与されています。
Dify の設定画面から Amazon Bedrock をモデルプロバイダーとして追加できます。
