module.exports = {
  testEnvironment: 'node',
  roots: ['<rootDir>/cdk'],
  testMatch: ['**/*.spec.ts'],
  transform: {
    '^.+\\.tsx?$': 'ts-jest',
  },
};
