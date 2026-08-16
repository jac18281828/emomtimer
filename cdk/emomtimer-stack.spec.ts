import * as cdk from 'aws-cdk-lib';
import { Match, Template } from 'aws-cdk-lib/assertions';

import { EmomTimerStack } from './emomtimer-stack';

describe('EmomTimerStack', () => {
  const app = new cdk.App();
  const stack = new EmomTimerStack(app, 'TestEmomTimerStack', {
    env: {
      account: '504242000181',
      region: 'us-east-1',
    },
  });

  const template = Template.fromStack(stack);

  it('creates exactly one of each core resource, including an owned bucket', () => {
    template.resourceCountIs('AWS::CloudFront::Distribution', 1);
    template.resourceCountIs('AWS::CertificateManager::Certificate', 1);
    // Owned bucket, provisioned by this stack rather than imported — a
    // regression to an imported bucket would drop this resource entirely.
    template.resourceCountIs('AWS::S3::Bucket', 1);
  });

  it('requests the certificate for the site subdomain', () => {
    template.hasResourceProperties('AWS::CertificateManager::Certificate', {
      DomainName: 'emomtimer.2ad.com',
    });
  });

  it('secures the origin with Origin Access Control, not legacy OAI', () => {
    const oacResources = template.findResources('AWS::CloudFront::OriginAccessControl');
    const oacLogicalIds = Object.keys(oacResources);
    expect(oacLogicalIds).toHaveLength(1);

    template.hasResourceProperties('AWS::CloudFront::Distribution', {
      DistributionConfig: Match.objectLike({
        Origins: Match.arrayWith([
          Match.objectLike({
            OriginAccessControlId: {
              'Fn::GetAtt': [oacLogicalIds[0], 'Id'],
            },
          }),
        ]),
      }),
    });

    template.resourceCountIs('AWS::CloudFront::CloudFrontOriginAccessIdentity', 0);
  });

  it('destroys the bucket, encrypts it, and blocks all public access', () => {
    template.hasResource('AWS::S3::Bucket', {
      DeletionPolicy: 'Delete',
      UpdateReplacePolicy: 'Delete',
      Properties: Match.objectLike({
        BucketName: 'emomtimer-us-east-1-504242000181',
        BucketEncryption: {
          ServerSideEncryptionConfiguration: [
            {
              ServerSideEncryptionByDefault: {
                SSEAlgorithm: 'AES256',
              },
            },
          ],
        },
        PublicAccessBlockConfiguration: {
          BlockPublicAcls: true,
          BlockPublicPolicy: true,
          IgnorePublicAcls: true,
          RestrictPublicBuckets: true,
        },
      }),
    });
  });

  it('empties the bucket before teardown via the auto-delete custom resource', () => {
    const bucketLogicalIds = Object.keys(template.findResources('AWS::S3::Bucket'));
    expect(bucketLogicalIds).toHaveLength(1);
    const [bucketLogicalId] = bucketLogicalIds;

    const handlerRoleLogicalIds = Object.keys(template.findResources('AWS::IAM::Role'));
    expect(handlerRoleLogicalIds).toHaveLength(1);
    const [handlerRoleLogicalId] = handlerRoleLogicalIds;

    // BucketName must reference this stack's own bucket, not merely exist —
    // a custom resource pointed at a different bucket would pass a bare
    // resource-count check while leaving this bucket un-emptied.
    template.hasResourceProperties('Custom::S3AutoDeleteObjects', {
      BucketName: { Ref: bucketLogicalId },
    });

    // The handler role needs an explicit delete grant on this bucket, or
    // the custom resource exists but fails at runtime with AccessDenied.
    template.hasResourceProperties('AWS::S3::BucketPolicy', {
      Bucket: { Ref: bucketLogicalId },
      PolicyDocument: {
        Statement: Match.arrayWith([
          Match.objectLike({
            Effect: 'Allow',
            Action: Match.arrayWith(['s3:DeleteObject*']),
            Principal: {
              AWS: {
                'Fn::GetAtt': [handlerRoleLogicalId, 'Arn'],
              },
            },
          }),
        ]),
      },
    });
  });

  it('scopes the bucket policy to CloudFront by Sid, with the expected effects', () => {
    template.hasResourceProperties('AWS::S3::BucketPolicy', {
      Bucket: { Ref: Match.anyValue() },
      PolicyDocument: {
        Statement: Match.arrayWith([
          Match.objectLike({
            Sid: 'AllowCloudFrontServicePrincipalReadOnly',
            Effect: 'Allow',
            Action: 's3:GetObject',
          }),
          Match.objectLike({
            Sid: 'DenyDirectS3ReadForObjects',
            Effect: 'Deny',
            Action: 's3:GetObject',
          }),
        ]),
      },
    });
  });

  it('manages the bucket policy as a single resource in this stack', () => {
    // S3 allows one policy per bucket; a second AWS::S3::BucketPolicy
    // synthesizes cleanly and then fails at deploy.
    template.resourceCountIs('AWS::S3::BucketPolicy', 1);
  });

  it('denies non-TLS requests via the enforceSSL statement', () => {
    template.hasResourceProperties('AWS::S3::BucketPolicy', {
      PolicyDocument: {
        Statement: Match.arrayWith([
          Match.objectLike({
            Effect: 'Deny',
            Principal: { AWS: '*' },
            Condition: {
              Bool: { 'aws:SecureTransport': 'false' },
            },
          }),
        ]),
      },
    });
  });

  it('creates A and AAAA alias records in the shared hosted zone', () => {
    template.resourceCountIs('AWS::Route53::RecordSet', 2);

    template.hasResourceProperties('AWS::Route53::RecordSet', {
      Name: 'emomtimer.2ad.com.',
      Type: 'A',
      HostedZoneId: 'Z09862671HYH6ZFKNPGNL',
    });

    template.hasResourceProperties('AWS::Route53::RecordSet', {
      Name: 'emomtimer.2ad.com.',
      Type: 'AAAA',
      HostedZoneId: 'Z09862671HYH6ZFKNPGNL',
    });
  });

  it('does not manage the shared hosted zone itself', () => {
    template.resourceCountIs('AWS::Route53::HostedZone', 0);
  });

  it('configures SPA behavior for the CloudFront distribution', () => {
    template.hasResourceProperties('AWS::CloudFront::Distribution', {
      DistributionConfig: {
        Aliases: Match.arrayWith(['emomtimer.2ad.com']),
        DefaultRootObject: 'index.html',
        CustomErrorResponses: Match.arrayWith([
          Match.objectLike({
            ErrorCode: 403,
            ResponseCode: 200,
            ResponsePagePath: '/index.html',
          }),
          Match.objectLike({
            ErrorCode: 404,
            ResponseCode: 200,
            ResponsePagePath: '/index.html',
          }),
        ]),
      },
    });
  });

  it('refuses to synthesize outside us-east-1', () => {
    expect(
      () =>
        new EmomTimerStack(new cdk.App(), 'WrongRegionEmomTimerStack', {
          env: {
            account: '504242000181',
            region: 'us-east-2',
          },
        }),
    ).toThrow(/us-east-1/);
  });
});
