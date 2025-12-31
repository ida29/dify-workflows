#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { DifyStack } from '../lib/dify-stack';

const app = new cdk.App();

// コンテキストから設定を取得
const allowedIp = app.node.tryGetContext('allowedIp') || '0.0.0.0/0';

new DifyStack(app, 'DifyStack', {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: process.env.CDK_DEFAULT_REGION,
  },
  allowedIp,
});
