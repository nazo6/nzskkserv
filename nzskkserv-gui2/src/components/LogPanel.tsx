import { useState, useEffect, useRef, useMemo, type FC } from "react";
import {
  Search,
  Trash2,
  ArrowDownToLine,
  Terminal,
} from "lucide-react";
import { Input } from "./ui/input";
import { Button } from "./ui/button";
import { Badge } from "./ui/badge";
import type { FormattedLogEntry, LogData } from "../types";

interface LogPanelProps {
  logs: FormattedLogEntry[];
  onClearLogs: () => void;
}

export const LogPanel: FC<LogPanelProps> = ({ logs, onClearLogs }) => {
  const [searchQuery, setSearchQuery] = useState("");
  const [levelFilter, setLevelFilter] = useState<string>("ALL");
  const [autoScroll, setAutoScroll] = useState(true);
  const scrollRef = useRef<HTMLDivElement>(null);

  const filteredLogs = useMemo(() => {
    return logs.filter((log) => {
      // Level filter
      if (levelFilter !== "ALL" && log.level.toUpperCase() !== levelFilter) {
        return false;
      }
      // Search filter
      if (searchQuery.trim() !== "") {
        const query = searchQuery.toLowerCase();
        const content = formatLogDataContent(log.data).toLowerCase();
        const target = log.target.toLowerCase();
        if (!content.includes(query) && !target.includes(query)) {
          return false;
        }
      }
      return true;
    });
  }, [logs, searchQuery, levelFilter]);

  // Handle auto-scroll to bottom
  useEffect(() => {
    if (autoScroll && scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [filteredLogs, autoScroll]);

  return (
    <div className="flex flex-col h-full overflow-hidden bg-slate-950/40">
      {/* Toolbar */}
      <div className="flex flex-wrap items-center justify-between gap-3 p-3 border-b border-slate-800/80 bg-slate-900/30">
        <div className="flex items-center gap-2 flex-1 min-w-[200px] max-w-sm">
          <div className="relative w-full">
            <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-slate-500" />
            <Input
              type="text"
              placeholder="Search in logs..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-8 h-8 text-xs bg-slate-900/90 border-slate-800"
            />
          </div>
        </div>

        <div className="flex items-center gap-2">
          {/* Level filter tabs */}
          <div className="flex items-center p-0.5 rounded-lg bg-slate-900 border border-slate-800 text-xs">
            {["ALL", "INFO", "WARN", "ERROR"].map((lvl) => (
              <button
                key={lvl}
                onClick={() => setLevelFilter(lvl)}
                className={`px-2 py-1 rounded-md font-medium transition-colors cursor-pointer ${
                  levelFilter === lvl
                    ? "bg-slate-800 text-white shadow-sm"
                    : "text-slate-400 hover:text-slate-200"
                }`}
              >
                {lvl}
              </button>
            ))}
          </div>

          {/* Auto scroll toggle */}
          <Button
            variant={autoScroll ? "secondary" : "outline"}
            size="sm"
            onClick={() => setAutoScroll(!autoScroll)}
            className="h-8 px-2.5 text-xs gap-1.5"
            title="Toggle Auto Scroll"
          >
            <ArrowDownToLine className={`w-3.5 h-3.5 ${autoScroll ? "text-indigo-400" : "text-slate-400"}`} />
            <span className="hidden sm:inline">Auto-scroll</span>
          </Button>

          {/* Clear button */}
          <Button
            variant="ghost"
            size="sm"
            onClick={onClearLogs}
            className="h-8 px-2 text-xs text-slate-400 hover:text-rose-400"
            title="Clear Log History"
          >
            <Trash2 className="w-3.5 h-3.5" />
          </Button>
        </div>
      </div>

      {/* Logs Table */}
      <div ref={scrollRef} className="flex-1 overflow-auto p-2 font-mono text-xs">
        {filteredLogs.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-slate-500 gap-2 select-none py-16">
            <Terminal className="w-8 h-8 stroke-[1.5] opacity-40" />
            <p className="text-sm">No log entries found</p>
          </div>
        ) : (
          <table className="w-full border-collapse">
            <thead>
              <tr className="border-b border-slate-800/60 text-slate-400 text-left text-[11px]">
                <th className="py-1 px-2.5 w-36 font-semibold">Time</th>
                <th className="py-1 px-2.5 w-20 font-semibold">Level</th>
                <th className="py-1 px-2.5 w-40 font-semibold">Target</th>
                <th className="py-1 px-2.5 font-semibold">Message</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-800/40">
              {filteredLogs.map((log) => (
                <tr
                  key={log.id}
                  className="hover:bg-slate-900/60 transition-colors group"
                >
                  <td className="py-1.5 px-2.5 text-slate-500 text-[11px] whitespace-nowrap">
                    {log.time}
                  </td>
                  <td className="py-1.5 px-2.5 whitespace-nowrap">
                    <LogLevelBadge level={log.level} />
                  </td>
                  <td className="py-1.5 px-2.5 text-slate-400 truncate max-w-[160px] text-[11px]" title={log.target}>
                    {log.target}
                  </td>
                  <td className="py-1.5 px-2.5 text-slate-200">
                    <LogContentDisplay data={log.data} />
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
};

const LogLevelBadge: FC<{ level: string }> = ({ level }) => {
  const upper = level.toUpperCase();
  switch (upper) {
    case "ERROR":
      return (
        <Badge variant="destructive" className="px-1.5 py-0 text-[10px]">
          ERR
        </Badge>
      );
    case "WARN":
      return (
        <Badge variant="warning" className="px-1.5 py-0 text-[10px]">
          WARN
        </Badge>
      );
    case "INFO":
      return (
        <Badge variant="default" className="px-1.5 py-0 text-[10px]">
          INFO
        </Badge>
      );
    default:
      return (
        <Badge variant="secondary" className="px-1.5 py-0 text-[10px]">
          {upper}
        </Badge>
      );
  }
};

const LogContentDisplay: FC<{ data: LogData }> = ({ data }) => {
  if (data.type === "ConvertInput") {
    return (
      <div className="flex items-center gap-1.5 text-sky-300">
        <span className="px-1.5 py-0.5 rounded bg-sky-950/70 border border-sky-800/50 text-[10px] text-sky-400 font-bold tracking-wider">
          SKK IN
        </span>
        <span className="font-semibold">{data.content}</span>
      </div>
    );
  }

  if (data.type === "ConvertOutput") {
    return (
      <div className="flex items-center gap-1.5 text-emerald-300">
        <span className="px-1.5 py-0.5 rounded bg-emerald-950/70 border border-emerald-800/50 text-[10px] text-emerald-400 font-bold tracking-wider">
          SKK OUT
        </span>
        <span>{data.content}</span>
      </div>
    );
  }

  return <span className="text-slate-300">{data.content}</span>;
};

function formatLogDataContent(data: LogData): string {
  return data.content;
}
