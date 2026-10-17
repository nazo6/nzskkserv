export type Encoding = "Utf8" | "Eucjp";

export type DictFormat = "Skk" | "Mozc";

export type DictDef = {
  path?: string;
  url?: string;
  encoding: Encoding;
  format: DictFormat;
};

export type Config = {
  enable_google_cgi: boolean;
  server_encoding: Encoding;
  port: number;
  dicts: DictDef[];
};

export type ServerState = {
  running: boolean;
  config: Config;
};

export type LogData =
  | { type: "Message"; content: string }
  | { type: "ConvertInput"; content: string }
  | { type: "ConvertOutput"; content: string };

export type FormattedLogEntry = {
  id: number;
  time: string;
  level: string;
  target: string;
  data: LogData;
};

