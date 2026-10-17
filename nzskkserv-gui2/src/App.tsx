import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Toaster, toast } from "sonner";
import { Terminal, Settings as SettingsIcon } from "lucide-react";
import { Header } from "./components/Header";
import { LogPanel } from "./components/LogPanel";
import { ConfigPanel } from "./components/ConfigPanel";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "./components/ui/tabs";
import type { ServerState, FormattedLogEntry } from "./types";

export function App() {
  const [serverState, setServerState] = useState<ServerState | null>(null);
  const [logs, setLogs] = useState<FormattedLogEntry[]>([]);
  const [activeTab, setActiveTab] = useState<string>("logs");

  useEffect(() => {
    // Load initial server state
    invoke<ServerState>("get_server_state")
      .then(setServerState)
      .catch((e) => console.error("Failed to load server state:", e));

    // Load initial recent logs
    invoke<FormattedLogEntry[]>("get_recent_logs")
      .then(setLogs)
      .catch((e) => console.error("Failed to load initial logs:", e));

    // Listen for server state changes
    const unlistenState = listen<ServerState>("server-state-changed", (event) => {
      setServerState(event.payload);
    });

    // Listen for real-time log entries
    const unlistenLog = listen<FormattedLogEntry>("log-entry", (event) => {
      setLogs((prev) => {
        const next = [...prev, event.payload];
        if (next.length > 500) {
          return next.slice(next.length - 500);
        }
        return next;
      });
    });

    return () => {
      unlistenState.then((fn) => fn());
      unlistenLog.then((fn) => fn());
    };
  }, []);

  const handleClearLogs = () => {
    setLogs([]);
    toast.info("Log display cleared");
  };

  return (
    <div className="flex flex-col h-full bg-slate-950 text-slate-100 select-none overflow-hidden">
      {/* App Header */}
      <Header serverState={serverState} onStateChange={setServerState} />

      {/* Main Tabbed Content */}
      <Tabs
        value={activeTab}
        onValueChange={setActiveTab}
        className="flex flex-col flex-1 overflow-hidden"
      >
        <div className="flex items-center justify-between px-5 py-2 border-b border-slate-800 bg-slate-900/50">
          <TabsList className="bg-slate-900 border-slate-800">
            <TabsTrigger value="logs" className="gap-1.5 text-xs">
              <Terminal className="w-3.5 h-3.5" />
              Logs
              {logs.length > 0 && (
                <span className="ml-1 px-1.5 py-0.2 rounded-full bg-slate-800 text-[10px] text-slate-400 font-mono">
                  {logs.length}
                </span>
              )}
            </TabsTrigger>
            <TabsTrigger value="settings" className="gap-1.5 text-xs">
              <SettingsIcon className="w-3.5 h-3.5" />
              Settings
            </TabsTrigger>
          </TabsList>
        </div>

        <TabsContent value="logs" className="m-0 flex-1 overflow-hidden">
          <LogPanel logs={logs} onClearLogs={handleClearLogs} />
        </TabsContent>

        <TabsContent value="settings" className="m-0 flex-1 overflow-hidden">
          <ConfigPanel
            serverState={serverState}
            onStateChange={setServerState}
          />
        </TabsContent>
      </Tabs>

      {/* Modern Toast Notifications */}
      <Toaster
        theme="dark"
        position="bottom-right"
        richColors
        closeButton
      />
    </div>
  );
}

export default App;
