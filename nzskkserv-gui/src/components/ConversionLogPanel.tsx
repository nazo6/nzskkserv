import { useState, useEffect, useRef, useMemo, type FC } from "react";
import {
  Search,
  Trash2,
  ArrowDownToLine,
  ArrowRight,
  Languages,
  Copy,
  Check,
} from "lucide-react";
import { Input } from "./ui/input";
import { Button } from "./ui/button";
import { Badge } from "./ui/badge";
import type { FormattedLogEntry, CandidateItem } from "../types";

interface ConversionLogPanelProps {
  logs: FormattedLogEntry[];
  onClearLogs: () => void;
}

interface NormalizedConversionEntry {
  id: number;
  time: string;
  input: string;
  candidates: CandidateItem[];
}

export const ConversionLogPanel: FC<ConversionLogPanelProps> = ({
  logs,
  onClearLogs,
}) => {
  const [searchQuery, setSearchQuery] = useState("");
  const [autoScroll, setAutoScroll] = useState(true);
  const [copiedCandidate, setCopiedCandidate] = useState<string | null>(null);
  const [expandedIds, setExpandedIds] = useState<Set<number>>(new Set());
  const scrollRef = useRef<HTMLDivElement>(null);

  // Normalize log entries to conversion items
  const conversionEntries = useMemo(() => {
    const list: NormalizedConversionEntry[] = [];

    // Pending input for pairing if old ConvertInput / ConvertOutput format arrives
    let pendingInput: { id: number; time: string; input: string } | null = null;

    for (const log of logs) {
      if (log.data.type === "Conversion") {
        list.push({
          id: log.id,
          time: log.time,
          input: log.data.content.input,
          candidates: log.data.content.candidates,
        });
      } else if (log.data.type === "ConvertInput") {
        const cleanInput = log.data.content.replace(/^"|"$/g, "");
        pendingInput = { id: log.id, time: log.time, input: cleanInput };
      } else if (log.data.type === "ConvertOutput") {
        const candidates = parseRawCandidates(log.data.content);
        if (pendingInput) {
          list.push({
            id: pendingInput.id,
            time: pendingInput.time,
            input: pendingInput.input,
            candidates,
          });
          pendingInput = null;
        } else {
          list.push({
            id: log.id,
            time: log.time,
            input: "(unknown)",
            candidates,
          });
        }
      }
    }

    return list;
  }, [logs]);

  const filteredEntries = useMemo(() => {
    if (searchQuery.trim() === "") {
      return conversionEntries;
    }
    const query = searchQuery.toLowerCase();
    return conversionEntries.filter((entry) => {
      if (entry.input.toLowerCase().includes(query)) {
        return true;
      }
      return entry.candidates.some(
        (c) =>
          c.candidate.toLowerCase().includes(query) ||
          (c.description && c.description.toLowerCase().includes(query))
      );
    });
  }, [conversionEntries, searchQuery]);

  // Handle auto-scroll to bottom
  useEffect(() => {
    if (autoScroll && scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [filteredEntries, autoScroll]);

  const handleCopy = (text: string) => {
    navigator.clipboard.writeText(text);
    setCopiedCandidate(text);
    setTimeout(() => {
      setCopiedCandidate((prev) => (prev === text ? null : prev));
    }, 1500);
  };

  const toggleExpand = (id: number) => {
    setExpandedIds((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  return (
    <div className="flex flex-col h-full overflow-hidden bg-slate-950/40">
      {/* Toolbar */}
      <div className="flex flex-wrap items-center justify-between gap-3 p-3 border-b border-slate-800/80 bg-slate-900/30">
        <div className="flex items-center gap-2 flex-1 min-w-[200px] max-w-sm">
          <div className="relative w-full">
            <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-slate-500" />
            <Input
              type="text"
              placeholder="Search input or candidates..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-8 h-8 text-xs bg-slate-900/90 border-slate-800"
            />
          </div>
        </div>

        <div className="flex items-center gap-2">
          {/* Conversions count badge */}
          <span className="text-xs text-slate-400 font-mono hidden sm:inline">
            {filteredEntries.length} {filteredEntries.length === 1 ? "entry" : "entries"}
          </span>

          {/* Auto scroll toggle */}
          <Button
            variant={autoScroll ? "secondary" : "outline"}
            size="sm"
            onClick={() => setAutoScroll(!autoScroll)}
            className="h-8 px-2.5 text-xs gap-1.5"
            title="Toggle Auto Scroll"
          >
            <ArrowDownToLine
              className={`w-3.5 h-3.5 ${
                autoScroll ? "text-indigo-400" : "text-slate-400"
              }`}
            />
            <span className="hidden sm:inline">Auto-scroll</span>
          </Button>

          {/* Clear button */}
          <Button
            variant="ghost"
            size="sm"
            onClick={onClearLogs}
            className="h-8 px-2 text-xs text-slate-400 hover:text-rose-400"
            title="Clear Conversion History"
          >
            <Trash2 className="w-3.5 h-3.5" />
          </Button>
        </div>
      </div>

      {/* Conversion List */}
      <div ref={scrollRef} className="flex-1 overflow-auto p-3 font-mono text-xs">
        {filteredEntries.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-slate-500 gap-2.5 select-none py-16">
            <Languages className="w-8 h-8 stroke-[1.5] opacity-40" />
            <p className="text-sm font-medium">No conversion logs found</p>
            <p className="text-xs text-slate-600">
              Conversions requested by your SKK client will appear here in real time.
            </p>
          </div>
        ) : (
          <div className="flex flex-col gap-1.5">
            {filteredEntries.map((entry) => {
              const isExpanded = expandedIds.has(entry.id);
              const maxInitial = 8;
              const hasMore = entry.candidates.length > maxInitial;
              const visibleCandidates =
                isExpanded || !hasMore
                  ? entry.candidates
                  : entry.candidates.slice(0, maxInitial);

              return (
                <div
                  key={entry.id}
                  className="flex flex-col sm:flex-row sm:items-center gap-2 p-2 rounded-lg bg-slate-900/40 border border-slate-800/60 hover:bg-slate-900/80 hover:border-slate-700/70 transition-colors"
                >
                  {/* Timestamp & Input header */}
                  <div className="flex items-center gap-2 shrink-0">
                    <span className="text-[11px] text-slate-500 whitespace-nowrap min-w-[70px]">
                      {entry.time.split(" ")[1] ?? entry.time}
                    </span>

                    {/* Word Key Badge */}
                    <span className="px-2 py-0.5 rounded font-bold text-xs bg-sky-950/70 border border-sky-800/60 text-sky-300 shadow-xs max-w-[130px] truncate" title={entry.input}>
                      {entry.input}
                    </span>

                    <ArrowRight className="w-3.5 h-3.5 text-slate-600 shrink-0" />
                  </div>

                  {/* Candidates */}
                  <div className="flex items-center flex-wrap gap-1.5 flex-1 min-w-0">
                    {entry.candidates.length === 0 ? (
                      <span className="text-slate-500 italic text-[11px]">
                        (no candidates)
                      </span>
                    ) : (
                      <>
                        {visibleCandidates.map((cand, idx) => {
                          const isCopied = copiedCandidate === cand.candidate;
                          return (
                            <button
                              key={idx}
                              onClick={() => handleCopy(cand.candidate)}
                              className="inline-flex items-center gap-1 px-2 py-0.5 rounded-md bg-slate-800/80 hover:bg-slate-700/80 border border-slate-700/70 text-slate-200 text-xs transition-colors cursor-pointer group/cand"
                              title={
                                cand.description
                                  ? `${cand.candidate} (${cand.description}) - Click to copy`
                                  : `${cand.candidate} - Click to copy`
                              }
                            >
                              <span className="font-medium text-slate-100">
                                {cand.candidate}
                              </span>
                              {cand.description && (
                                <span className="text-[10px] text-slate-400">
                                  ({cand.description})
                                </span>
                              )}
                              <span className="opacity-0 group-hover/cand:opacity-100 text-slate-400 transition-opacity ml-0.5">
                                {isCopied ? (
                                  <Check className="w-3 h-3 text-emerald-400" />
                                ) : (
                                  <Copy className="w-2.5 h-2.5" />
                                )}
                              </span>
                            </button>
                          );
                        })}

                        {hasMore && (
                          <button
                            onClick={() => toggleExpand(entry.id)}
                            className="px-1.5 py-0.5 rounded text-[10px] bg-slate-800 text-indigo-400 hover:text-indigo-300 hover:bg-slate-700 font-medium cursor-pointer transition-colors"
                          >
                            {isExpanded
                              ? "Show less"
                              : `+${entry.candidates.length - maxInitial} more`}
                          </button>
                        )}
                      </>
                    )}
                  </div>

                  {/* Total Candidates count */}
                  <div className="hidden sm:flex items-center shrink-0 self-center">
                    <Badge
                      variant="secondary"
                      className="text-[10px] px-1.5 py-0 text-slate-400 font-normal"
                    >
                      {entry.candidates.length}
                    </Badge>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
};

// Fallback parser in case logs contain raw Rust Debug or JSON string format
function parseRawCandidates(raw: string): CandidateItem[] {
  // If JSON format
  try {
    const parsed = JSON.parse(raw);
    if (Array.isArray(parsed)) {
      return parsed.map((item) => {
        if (typeof item === "string") {
          return { candidate: item };
        }
        if (item && typeof item === "object") {
          return {
            candidate: item.candidate ?? String(item),
            description: item.description ?? null,
          };
        }
        return { candidate: String(item) };
      });
    }
  } catch {
    // Not valid JSON, continue with regex parsing
  }

  // Rust Debug format: [Entry { candidate: "...", description: ... }, ...]
  const results: CandidateItem[] = [];
  const entryRegex = /candidate:\s*"([^"]+)"(?:,\s*description:\s*(?:Some\("([^"]+)"\)|None))?/g;
  let match;
  while ((match = entryRegex.exec(raw)) !== null) {
    results.push({
      candidate: match[1],
      description: match[2] ?? null,
    });
  }

  if (results.length > 0) {
    return results;
  }

  // Plain string or single candidate
  const cleaned = raw.replace(/^\[|\]$|^"|"$/g, "").trim();
  if (cleaned) {
    return [{ candidate: cleaned }];
  }

  return [];
}
