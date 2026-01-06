import * as cdk from 'aws-cdk-lib';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as iam from 'aws-cdk-lib/aws-iam';
import { Construct } from 'constructs';
import * as fs from 'fs';
import * as path from 'path';

interface DifyStackProps extends cdk.StackProps {
  allowedIp: string;
}

export class DifyStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: DifyStackProps) {
    super(scope, id, props);

    // VPC (パブリックサブネットのみでコスト削減)
    const vpc = new ec2.Vpc(this, 'DifyVpc', {
      maxAzs: 1, // 単一AZでコスト削減
      natGateways: 0, // NAT Gateway なし
      subnetConfiguration: [
        {
          cidrMask: 24,
          name: 'Public',
          subnetType: ec2.SubnetType.PUBLIC,
        },
      ],
    });

    // セキュリティグループ
    const securityGroup = new ec2.SecurityGroup(this, 'DifySecurityGroup', {
      vpc,
      description: 'Security group for Dify instance',
      allowAllOutbound: true,
    });

    // SSH アクセス (指定IPのみ)
    securityGroup.addIngressRule(
      ec2.Peer.ipv4(props.allowedIp),
      ec2.Port.tcp(22),
      'SSH access'
    );

    // HTTP アクセス (GitHub Actions等からのアクセスのため全開放)
    securityGroup.addIngressRule(
      ec2.Peer.anyIpv4(),
      ec2.Port.tcp(80),
      'HTTP access from anywhere'
    );

    // HTTPS アクセス (GitHub Actions等からのアクセスのため全開放)
    securityGroup.addIngressRule(
      ec2.Peer.anyIpv4(),
      ec2.Port.tcp(443),
      'HTTPS access from anywhere'
    );

    // IAM ロール (Bedrock アクセス用)
    const role = new iam.Role(this, 'DifyInstanceRole', {
      assumedBy: new iam.ServicePrincipal('ec2.amazonaws.com'),
      managedPolicies: [
        iam.ManagedPolicy.fromAwsManagedPolicyName('AmazonSSMManagedInstanceCore'),
        iam.ManagedPolicy.fromAwsManagedPolicyName('AmazonBedrockFullAccess'),
      ],
    });

    // SSM Parameter Store 読み取り権限（DuckDNSトークン用）
    role.addToPolicy(new iam.PolicyStatement({
      actions: ['ssm:GetParameter'],
      resources: [`arn:aws:ssm:${this.region}:${this.account}:parameter/dify/*`],
    }));

    // 最新の Amazon Linux 2023 AMI
    const ami = ec2.MachineImage.latestAmazonLinux2023({
      cpuType: ec2.AmazonLinuxCpuType.X86_64,
    });

    // User Data スクリプト
    const userDataScript = fs.readFileSync(
      path.join(__dirname, '../scripts/user-data.sh'),
      'utf8'
    );
    const userData = ec2.UserData.custom(userDataScript);

    // サブネット取得
    const subnet = vpc.publicSubnets[0];

    // オンデマンドインスタンス（停止→起動でデータ永続化）
    const instance = new ec2.Instance(this, 'DifyInstance', {
      vpc,
      vpcSubnets: { subnets: [subnet] },
      instanceType: ec2.InstanceType.of(ec2.InstanceClass.T3, ec2.InstanceSize.MEDIUM),
      machineImage: ami,
      securityGroup,
      role,
      userData,
      blockDevices: [
        {
          deviceName: '/dev/xvda',
          volume: ec2.BlockDeviceVolume.ebs(30, {
            volumeType: ec2.EbsDeviceVolumeType.GP3,
            encrypted: true,
            deleteOnTermination: true,
          }),
        },
      ],
      requireImdsv2: true,
    });

    // Elastic IP (固定IP)
    const eip = new ec2.CfnEIP(this, 'DifyEip', {
      domain: 'vpc',
    });

    new ec2.CfnEIPAssociation(this, 'DifyEipAssociation', {
      allocationId: eip.attrAllocationId,
      instanceId: instance.instanceId,
    });

    // Outputs
    new cdk.CfnOutput(this, 'InstanceId', {
      value: instance.instanceId,
      description: 'EC2 Instance ID',
    });

    new cdk.CfnOutput(this, 'PublicIp', {
      value: eip.attrPublicIp,
      description: 'Elastic IP address',
    });

    new cdk.CfnOutput(this, 'DifyUrl', {
      value: `http://${eip.attrPublicIp}`,
      description: 'Dify URL',
    });

    new cdk.CfnOutput(this, 'SshCommand', {
      value: `ssh ec2-user@${eip.attrPublicIp}`,
      description: 'SSH command (use Session Manager or add key pair)',
    });
  }
}
