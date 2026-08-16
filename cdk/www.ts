import * as cdk from 'aws-cdk-lib';

import { EmomTimerStack } from './emomtimer-stack';

const ACCOUNT = process.env.CDK_DEPLOY_ACCOUNT ?? process.env.CDK_DEFAULT_ACCOUNT ?? '504242000181';

const app = new cdk.App();

new EmomTimerStack(app, 'StackEmomTimer2adCom', {
  env: {
    account: ACCOUNT,
    region: 'us-east-1',
  },
});
