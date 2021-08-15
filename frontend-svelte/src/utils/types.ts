import type { IrItem } from '../../__generated__/ir_ts_types';

export const DATA_PATH = '/static/output.json';
export const KINDS: `${Lowercase<IrItem['kind']>}`[] = [
	'action',
	'instruction',
	'markdown',
	'dependencylist',
	'namedreflist',
	'namedref',
	'step',
	'command',
	'commanddefinition',
	'commandconfig',
	'config',
	'configdefinition'
];
