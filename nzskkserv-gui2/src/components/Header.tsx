import type { FC } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Play, Square, Power } from "lucide-react";
import { Button } from "./ui/button";
import { Badge } from "./ui/badge";
import type { ServerState } from "../types";

interface HeaderProps {
  serverState: ServerState | null;
  onStateChange: (newState: ServerState) => void;
}

export const Header: FC<HeaderProps> = ({ serverState, onStateChange }) => {
  const isRunning = serverState?.running ?? false;
  const port = serverState?.config?.port ?? 1178;

  const toggleServer = async () => {
    try {
      const newState = await invoke<ServerState>("set_server_running", {
        running: !isRunning,
      });
      onStateChange(newState);
    } catch (e) {
      console.error("Failed to toggle server state:", e);
    }
  };

  const quitApp = () => {
    window.close();
  };

  return (
    <header className="flex items-center justify-between px-5 py-3 border-b border-slate-800 bg-slate-950/80 backdrop-blur-md">
      <div className="flex items-center gap-3">
        <div className="flex items-center justify-center w-8 h-8 rounded-lg bg-indigo-600/20 border border-indigo-500/30 text-indigo-400 font-bold text-base shadow-sm shadow-indigo-950/40">
          SKK
        </div>
        <div>
          <div className="flex items-center gap-2">
            <h1 className="text-sm font-semibold tracking-wide text-slate-100">
              nzskkserv
            </h1>
            <span className="text-xs text-slate-500 font-mono">v0.2.7</span>
          </div>
          <div className="flex items-center gap-2 mt-0.5">
            {isRunning ? (
              <Badge variant="success" className="gap-1 py-0 px-2 text-[10px]">
                <span className="relative flex h-1.5 w-1.5">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                  <span className="relative inline-flex rounded-full h-1.5 w-1.5 bg-emerald-500"></span>
                </span>
                Running on port {port}
              </Badge>
            ) : (
              <Badge variant="secondary" className="gap-1 py-0 px-2 text-[10px] text-slate-400">
                <span className="inline-flex rounded-full h-1.5 w-1.5 bg-slate-500"></span>
                Stopped
              </Badge>
            )}
          </div>
        </div>
      </div>

      <div className="flex items-center gap-2">
        <Button
          variant={isRunning ? "destructive" : "success"}
          size="sm"
          onClick={toggleServer}
          className="gap-1.5 shadow-sm font-medium"
        >
          {isRunning ? (
            <>
              <Square className="w-3.5 h-3.5 fill-current" />
              Stop Server
            </>
          ) : (
            <>
              <Play className="w-3.5 h-3.5 fill-current" />
              Start Server
            </>
          )}
        </Button>

        <Button
          variant="outline"
          size="sm"
          onClick={quitApp}
          title="Quit Application"
          className="text-slate-400 hover:text-rose-400 hover:border-rose-900/60"
        >
          <Power className="w-3.5 h-3.5" />
          <span className="hidden sm:inline">Quit</span>
        </Button>
      </div>
    </header>
  );
};
