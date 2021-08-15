export type Ir = { items: IrItem [] };
export type IrItem = 
 | { kind: "Action"; fields: Action } 
 | { kind: "Instruction"; fields: Instruction } 
 | { kind: "Markdown"; fields: Markdown } 
 | { kind: "DependencyList"; fields: DependencyList } 
 | { kind: "NamedRefList"; fields: NamedRefList } 
 | { kind: "NamedRef"; fields: NamedRef } 
 | { kind: "Step"; fields: Step } 
 | { kind: "Command"; fields: Command } 
 | { kind: "CommandDefinition"; fields: CommandDefinition } 
 | { kind: "CommandConfig"; fields: CommandConfig } 
 | { kind: "Config"; fields: Config } 
 | { kind: "ConfigDefinition"; fields: ConfigDefinition };
export type Action = { name: string; content: IrItem [] };
export type Instruction = { name: string; loc: Location | null; content: IrItem [] };
export type Markdown = { content: string; loc: Location | null };
export type DependencyList = { content: IrItem []; loc: Location | null };
export type NamedRefList = { content: IrItem []; loc: Location | null };
export type NamedRef = { name: string; loc: Location | null };
export type Step = { name: string; content: IrItem [] };
export type Command = { name: string; content: IrItem [] };
export type CommandDefinition = { command: string; params: CommandParams };
export type CommandConfig = { config: string; params: CommandParams };
export type CommandParams = { language: InputLanguage | null };
export enum InputLanguage { toml = "toml", yaml = "yaml", shell = "shell" };
export type Config = { name: string; content: IrItem [] };
export type ConfigDefinition = { params: ConfigParams; config: string };
export type ConfigParams = { language: InputLanguage | null };
