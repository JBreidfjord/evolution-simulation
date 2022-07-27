module.exports = {
  'env': {
    'browser': true,
    'es2021': true,
    'node': true
  },
  'extends': [
    'eslint:recommended',
    'plugin:react/recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:import/recommended',
    'plugin:import/typescript'
  ],
  'globals': {
    React: true,
    JSX: true
  },
  'parser': '@typescript-eslint/parser',
  'parserOptions': {
    'ecmaFeatures': {
      'jsx': true
    },
    'ecmaVersion': 'latest',
    'sourceType': 'module',
  },
  'settings': {
    'import/resolver': {
      'node': {
        'extensions': ['.js', '.jsx', '.ts', '.tsx'],
        'moduleDirectory': ['node_modules', './']
      }
    }
  },
  'plugins': [
    'react',
    '@typescript-eslint'
  ],
  'overrides': [
    {
      'files': ['*.js', '*.jsx'],
      'rules': {
        '@typescript-eslint/explicit-function-return-type': 'off',
      }
    }
  ],
  'rules': {
    'indent': [
      'error',
      2
    ],
    'linebreak-style': [
      'error',
      'unix'
    ],
    'quotes': [
      'error',
      'single'
    ],
    'semi': [
      'error',
      'always'
    ],
    '@typescript-eslint/explicit-function-return-type': ['error', { allowExpressions: true }],
    'sort-imports': [
      'warn',
      {
        ignoreDeclarationSort: true,
        allowSeparatedGroups: true
      }
    ],
    'import/no-unresolved': 'error',
    'import/order': [
      'warn',
      {
        'groups': [
          'builtin',
          'external',
          ['internal', 'sibling', 'parent'],
          'index',
          'unknown'
        ],
        'newlines-between': 'always',
        'alphabetize': {
          order: 'asc',
          caseInsensitive: true
        }
      }
    ]
  }
};
