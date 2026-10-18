import { useState, useEffect, type FC } from "react";
import { invoke } from "@tauri-apps/api/core";
import { isEnabled, enable, disable } from "@tauri-apps/plugin-autostart";
import {
  Settings,
  BookOpen,
  RotateCcw,
  Save,
  CheckCircle2,
  FileCode,
} from "lucide-react";
import { toast } from "sonner";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "./ui/card";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { Switch } from "./ui/switch";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";
import { DictEditor } from "./DictEditor";
import type { Config, Encoding, ServerState } from "../types";

interface ConfigPanelProps {
  serverState: ServerState | null;
  onStateChange: (newState: ServerState) => void;
}

export const ConfigPanel: FC<ConfigPanelProps> = ({
  serverState,
  onStateChange,
}) => {
  const [config, setConfig] = useState<Config | null>(null);
  const [initialConfig, setInitialConfig] = useState<Config | null>(null);
  const [autoLaunch, setAutoLaunch] = useState(false);
  const [autoLaunchSupported, setAutoLaunchSupported] = useState(true);
  const [configPath, setConfigPath] = useState("");
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (serverState?.config) {
      setConfig(JSON.parse(JSON.stringify(serverState.config)));
      setInitialConfig(JSON.parse(JSON.stringify(serverState.config)));
    }
  }, [serverState]);

  useEffect(() => {
    // Check auto-launch support and status
    (async () => {
      try {
        const enabled = await isEnabled();
        setAutoLaunch(enabled);
      } catch (e) {
        console.warn("Auto launch not supported on this platform/environment:", e);
        setAutoLaunchSupported(false);
      }
    })();

    // Fetch config path
    invoke<string>("get_config_path")
      .then(setConfigPath)
      .catch((e) => console.error("Failed to get config path:", e));
  }, []);

  const handleAutoLaunchToggle = async (checked: boolean) => {
    try {
      if (checked) {
        await enable();
      } else {
        await disable();
      }
      setAutoLaunch(checked);
      toast.success(
        checked ? "Auto-start on login enabled" : "Auto-start on login disabled"
      );
    } catch (e: any) {
      console.error("Failed to toggle auto launch:", e);
      toast.error("Failed to update auto launch state: " + String(e));
    }
  };

  const handleApply = async () => {
    if (!config) return;
    setSaving(true);
    try {
      const newState = await invoke<ServerState>("save_config", {
        newConfig: config,
      });
      onStateChange(newState);
      setInitialConfig(JSON.parse(JSON.stringify(config)));
      toast.success("Settings saved and applied to server!");
    } catch (e: any) {
      console.error("Failed to save config:", e);
      toast.error("Failed to save settings: " + String(e));
    } finally {
      setSaving(false);
    }
  };

  const handleReset = () => {
    if (initialConfig) {
      setConfig(JSON.parse(JSON.stringify(initialConfig)));
      toast.info("Changes reset to current saved settings");
    }
  };

  if (!config) {
    return (
      <div className="flex items-center justify-center h-full text-slate-500">
        Loading settings...
      </div>
    );
  }

  const hasChanges =
    JSON.stringify(config) !== JSON.stringify(initialConfig);

  return (
    <div className="flex flex-col h-full overflow-hidden">
      <div className="flex-1 overflow-y-auto overscroll-contain p-5 space-y-6 max-w-4xl mx-auto w-full">
        {/* General Settings Card */}
        <Card>
          <CardHeader>
            <div className="flex items-center gap-2">
              <Settings className="w-4 h-4 text-indigo-400" />
              <CardTitle>General Server Settings</CardTitle>
            </div>
            <CardDescription>
              Basic network and encoding configurations for nzskkserv
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {/* Port */}
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-slate-300">
                  Listening Port
                </label>
                <Input
                  type="number"
                  value={config.port}
                  onChange={(e) =>
                    setConfig({ ...config, port: parseInt(e.target.value) || 1178 })
                  }
                  className="bg-slate-900/80 border-slate-700/60"
                />
                <p className="text-[11px] text-slate-500">
                  Standard SKK server port is 1178.
                </p>
              </div>

              {/* Server Encoding */}
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-slate-300">
                  Server Protocol Encoding
                </label>
                <Select
                  value={config.server_encoding}
                  onValueChange={(val: Encoding) =>
                    setConfig({ ...config, server_encoding: val })
                  }
                >
                  <SelectTrigger className="bg-slate-900/80 border-slate-700/60">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="Utf8">UTF-8</SelectItem>
                    <SelectItem value="Eucjp">EUC-JP</SelectItem>
                  </SelectContent>
                </Select>
                <p className="text-[11px] text-slate-500">
                  Encoding used for communication with SKK client.
                </p>
              </div>
            </div>

            <div className="pt-2 border-t border-slate-800/80 space-y-4">
              {/* Google CGI Toggle */}
              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <div className="text-xs font-medium text-slate-200">
                    Google Japanese Input CGI Completion
                  </div>
                  <div className="text-[11px] text-slate-400">
                    Query Google CGI when candidate is not found in dictionaries.
                  </div>
                </div>
                <Switch
                  checked={config.enable_google_cgi}
                  onCheckedChange={(checked) =>
                    setConfig({ ...config, enable_google_cgi: checked })
                  }
                />
              </div>

              {/* Auto Launch Toggle */}
              {autoLaunchSupported && (
                <div className="flex items-center justify-between pt-3 border-t border-slate-800/60">
                  <div className="space-y-0.5">
                    <div className="text-xs font-medium text-slate-200">
                      Launch on System Startup
                    </div>
                    <div className="text-[11px] text-slate-400">
                      Start nzskkserv automatically in the background on login.
                    </div>
                  </div>
                  <Switch
                    checked={autoLaunch}
                    onCheckedChange={handleAutoLaunchToggle}
                  />
                </div>
              )}
            </div>

            {/* Config file path */}
            {configPath && (
              <div className="pt-3 border-t border-slate-800/60 flex items-center gap-2 text-[11px] text-slate-500 font-mono">
                <FileCode className="w-3.5 h-3.5 shrink-0 text-slate-400" />
                <span className="truncate" title={configPath}>
                  Config file: {configPath}
                </span>
              </div>
            )}
          </CardContent>
        </Card>

        {/* Dictionaries Card */}
        <Card>
          <CardHeader>
            <div className="flex items-center gap-2">
              <BookOpen className="w-4 h-4 text-indigo-400" />
              <CardTitle>Dictionaries (辞書設定)</CardTitle>
            </div>
            <CardDescription>
              Configure local SKK dictionary files or online auto-cached dictionaries.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <DictEditor
              dicts={config.dicts}
              onChange={(newDicts) => setConfig({ ...config, dicts: newDicts })}
            />
          </CardContent>
        </Card>
      </div>

      {/* Floating Action Bar */}
      <div className="flex items-center justify-between px-6 py-3 border-t border-slate-800 bg-slate-950">
        <div className="text-xs text-slate-400">
          {hasChanges ? (
            <span className="text-amber-400 flex items-center gap-1.5 font-medium">
              <span className="h-1.5 w-1.5 rounded-full bg-amber-400"></span>
              You have unsaved changes
            </span>
          ) : (
            <span className="text-slate-500 flex items-center gap-1.5">
              <CheckCircle2 className="w-3.5 h-3.5 text-emerald-500" />
              Settings are up to date
            </span>
          )}
        </div>

        <div className="flex items-center gap-2">
          <Button
            variant="ghost"
            size="sm"
            onClick={handleReset}
            disabled={!hasChanges || saving}
            className="text-xs gap-1.5 text-slate-400 hover:text-slate-200"
          >
            <RotateCcw className="w-3.5 h-3.5" />
            Reset
          </Button>

          <Button
            variant={hasChanges ? "default" : "secondary"}
            size="sm"
            onClick={handleApply}
            disabled={!hasChanges || saving}
            className="text-xs gap-1.5 shadow-sm"
          >
            <Save className="w-3.5 h-3.5" />
            {saving ? "Applying..." : "Save & Apply"}
          </Button>
        </div>
      </div>
    </div>
  );
};
