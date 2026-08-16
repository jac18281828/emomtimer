import * as cdk from 'aws-cdk-lib';
import * as acm from 'aws-cdk-lib/aws-certificatemanager';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import * as origins from 'aws-cdk-lib/aws-cloudfront-origins';
import * as iam from 'aws-cdk-lib/aws-iam';
import { aws_route53 as r53 } from 'aws-cdk-lib';
import * as r53t from 'aws-cdk-lib/aws-route53-targets';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { Construct } from 'constructs';

const ROOT_DOMAIN = '2ad.com';
const SUBDOMAIN = 'emomtimer.2ad.com';
const ROOT_ZONE_ID = 'Z09862671HYH6ZFKNPGNL';
const BUCKET_NAME = 'emomtimer-us-east-1-504242000181';
const RECORD_NAME = 'emomtimer';

export class EmomTimerStack extends cdk.Stack {
  public readonly distribution: cloudfront.Distribution;
  public readonly bucket: s3.Bucket;

  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const stackRegion = cdk.Stack.of(this).region;
    if (!cdk.Token.isUnresolved(stackRegion) && stackRegion !== 'us-east-1') {
      throw new Error('EmomTimerStack must be deployed in us-east-1 for CloudFront ACM certificates.');
    }

    // The zone is created and managed outside CloudFormation and shared by
    // every 2ad.com site, so it is imported read-only and never mutated here.
    const zone = r53.HostedZone.fromHostedZoneAttributes(this, 'RootZone', {
      hostedZoneId: ROOT_ZONE_ID,
      zoneName: ROOT_DOMAIN,
    });

    const certificate = new acm.Certificate(this, 'EmomTimerCertificate', {
      domainName: SUBDOMAIN,
      validation: acm.CertificateValidation.fromDns(zone),
    });

    // A retained bucket would survive cdk destroy and then fail the next
    // cdk deploy on the globally unique name; the site content is rebuilt
    // from source, so destroying it costs nothing.
    this.bucket = new s3.Bucket(this, 'EmomTimerOriginBucket', {
      bucketName: BUCKET_NAME,
      blockPublicAccess: s3.BlockPublicAccess.BLOCK_ALL,
      enforceSSL: true,
      encryption: s3.BucketEncryption.S3_MANAGED,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
    });

    this.distribution = new cloudfront.Distribution(this, 'EmomTimerDistribution', {
      defaultRootObject: 'index.html',
      certificate: certificate,
      domainNames: [SUBDOMAIN],
      minimumProtocolVersion: cloudfront.SecurityPolicyProtocol.TLS_V1_2_2021,
      httpVersion: cloudfront.HttpVersion.HTTP2_AND_3,
      priceClass: cloudfront.PriceClass.PRICE_CLASS_100,
      enableIpv6: true,
      defaultBehavior: {
        origin: origins.S3BucketOrigin.withOriginAccessControl(this.bucket),
        viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        allowedMethods: cloudfront.AllowedMethods.ALLOW_GET_HEAD_OPTIONS,
        cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        compress: true,
      },
      errorResponses: [
        {
          httpStatus: 403,
          responseHttpStatus: 200,
          responsePagePath: '/index.html',
          ttl: cdk.Duration.minutes(5),
        },
        {
          httpStatus: 404,
          responseHttpStatus: 200,
          responsePagePath: '/index.html',
          ttl: cdk.Duration.minutes(5),
        },
      ],
    });

    const distributionArn = `arn:${cdk.Aws.PARTITION}:cloudfront::${cdk.Aws.ACCOUNT_ID}:distribution/${this.distribution.distributionId}`;

    // Owned bucket: S3BucketOrigin.withOriginAccessControl and enforceSSL
    // already attach statements to the bucket's single CDK-managed policy.
    // Add these directly rather than a second CfnBucketPolicy, which would
    // produce a second AWS::S3::BucketPolicy resource on the same bucket.
    this.bucket.addToResourcePolicy(
      new iam.PolicyStatement({
        sid: 'AllowCloudFrontServicePrincipalReadOnly',
        effect: iam.Effect.ALLOW,
        principals: [new iam.ServicePrincipal('cloudfront.amazonaws.com')],
        actions: ['s3:GetObject'],
        resources: [`${this.bucket.bucketArn}/*`],
        conditions: {
          StringEquals: {
            'AWS:SourceArn': distributionArn,
            'AWS:SourceAccount': cdk.Aws.ACCOUNT_ID,
          },
        },
      }),
    );

    this.bucket.addToResourcePolicy(
      new iam.PolicyStatement({
        sid: 'DenyDirectS3ReadForObjects',
        effect: iam.Effect.DENY,
        principals: [new iam.AnyPrincipal()],
        actions: ['s3:GetObject'],
        resources: [`${this.bucket.bucketArn}/*`],
        conditions: {
          StringNotEquals: {
            'AWS:SourceArn': distributionArn,
          },
        },
      }),
    );

    const cloudFrontTarget = r53.RecordTarget.fromAlias(new r53t.CloudFrontTarget(this.distribution));

    new r53.ARecord(this, 'EmomTimerARecord', {
      zone: zone,
      recordName: RECORD_NAME,
      target: cloudFrontTarget,
    });

    new r53.AaaaRecord(this, 'EmomTimerAaaaRecord', {
      zone: zone,
      recordName: RECORD_NAME,
      target: cloudFrontTarget,
    });

    new cdk.CfnOutput(this, 'DistributionId', {
      value: this.distribution.distributionId,
      description: 'Set this as CLOUDFRONT_DISTRIBUTION_ID in the emomtimer repository secrets.',
    });

    new cdk.CfnOutput(this, 'DistributionArn', {
      value: distributionArn,
      description: 'CloudFront distribution ARN referenced by this stack\'s origin bucket policy.',
    });
  }
}
