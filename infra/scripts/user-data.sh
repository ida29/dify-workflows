#!/bin/bash
set -ex

# ログ出力
exec > >(tee /var/log/user-data.log) 2>&1

echo "=== Starting Dify setup ==="

# システムアップデート
dnf update -y

# Docker インストール
dnf install -y docker git

# Docker サービス開始
systemctl enable docker
systemctl start docker

# ec2-user を docker グループに追加
usermod -aG docker ec2-user

# Docker Compose インストール
DOCKER_COMPOSE_VERSION="v2.32.4"
curl -L "https://github.com/docker/compose/releases/download/${DOCKER_COMPOSE_VERSION}/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose

# Dify ディレクトリ作成
DIFY_DIR="/opt/dify"
mkdir -p ${DIFY_DIR}
cd ${DIFY_DIR}

# Dify クローン (最新安定版)
git clone https://github.com/langgenius/dify.git .

# Docker ディレクトリに移動
cd docker

# 環境変数ファイルをコピー
cp .env.example .env

# SECRET_KEY を生成して設定
SECRET_KEY=$(openssl rand -base64 42)
sed -i "s|^SECRET_KEY=.*|SECRET_KEY=${SECRET_KEY}|" .env

# Dify 起動
docker-compose up -d

# DuckDNS 更新（SSM Parameter Store からトークン取得）
echo "=== Updating DuckDNS ==="
DUCKDNS_TOKEN=$(aws ssm get-parameter --name "/dify/duckdns-token" --with-decryption --query 'Parameter.Value' --output text --region ap-northeast-1 2>/dev/null || echo "")
if [ -n "$DUCKDNS_TOKEN" ]; then
  PUBLIC_IP=$(curl -s http://169.254.169.254/latest/meta-data/public-ipv4)
  curl -s "https://www.duckdns.org/update?domains=yida-dify&token=${DUCKDNS_TOKEN}&ip=${PUBLIC_IP}"
  echo ""
  echo "DuckDNS updated: yida-dify.duckdns.org -> ${PUBLIC_IP}"
fi

echo "=== Dify setup completed ==="
echo "Access Dify at http://yida-dify.duckdns.org"
