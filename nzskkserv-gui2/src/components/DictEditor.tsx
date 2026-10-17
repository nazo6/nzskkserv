import { useState, type FC } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import {
  FolderOpen,
  RefreshCw,
  Trash2,
  Plus,
  FileText,
  Globe,
  Loader2,
} from "lucide-react";
import { toast } from "sonner";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";
import type { DictDef, Encoding, DictFormat } from "../types";

interface DictEditorProps {
  dicts: DictDef[];
  onChange: (dicts: DictDef[]) => void;
}

export const DictEditor: FC<DictEditorProps> = ({ dicts, onChange }) => {
  const [refreshingIdx, setRefreshingIdx] = useState<number | null>(null);

  const handleTypeChange = (index: number, newType: "File" | "Url") => {
    const updated = [...dicts];
    const current = updated[index];
    if (newType === "File") {
      updated[index] = {
        path: current.url || "",
        encoding: current.encoding,
        format: current.format,
      };
    } else {
      updated[index] = {
        url: current.path || "https://",
        encoding: current.encoding,
        format: current.format,
      };
    }
    onChange(updated);
  };

  const handlePathOrUrlChange = (index: number, value: string) => {
    const updated = [...dicts];
    const current = updated[index];
    if (current.path !== undefined) {
      updated[index] = { ...current, path: value };
    } else {
      updated[index] = { ...current, url: value };
    }
    onChange(updated);
  };

  const handleBrowseFile = async (index: number) => {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        title: "Select SKK Dictionary File",
      });
      if (selected && typeof selected === "string") {
        handlePathOrUrlChange(index, selected);
      }
    } catch (e) {
      console.error("Failed to open file picker:", e);
      toast.error("Failed to open file selector dialog");
    }
  };

  const handleRefreshCache = async (index: number, url: string) => {
    if (!url || !url.startsWith("http")) {
      toast.error("Please enter a valid HTTP/HTTPS URL");
      return;
    }

    setRefreshingIdx(index);
    try {
      await invoke("refresh_dict_cache", { urlStr: url });
      toast.success("Dictionary cache updated successfully!", {
        description: url,
      });
    } catch (e: any) {
      console.error("Failed to refresh cache:", e);
      toast.error("Failed to update dictionary cache", {
        description: String(e),
      });
    } finally {
      setRefreshingIdx(null);
    }
  };

  const handleEncodingChange = (index: number, encoding: Encoding) => {
    const updated = [...dicts];
    updated[index] = { ...updated[index], encoding };
    onChange(updated);
  };

  const handleFormatChange = (index: number, format: DictFormat) => {
    const updated = [...dicts];
    updated[index] = { ...updated[index], format };
    onChange(updated);
  };

  const handleDelete = (index: number) => {
    const updated = dicts.filter((_, i) => i !== index);
    onChange(updated);
  };

  const handleAdd = () => {
    const newDict: DictDef = {
      path: "",
      encoding: "Utf8",
      format: "Skk",
    };
    onChange([...dicts, newDict]);
  };

  return (
    <div className="space-y-3">
      <div className="rounded-xl border border-slate-800 bg-slate-900/30 overflow-hidden">
        <table className="w-full text-left text-xs border-collapse">
          <thead>
            <tr className="border-b border-slate-800 bg-slate-900/70 text-slate-400 font-medium">
              <th className="py-2.5 px-3 w-28">Source</th>
              <th className="py-2.5 px-3">Path / URL</th>
              <th className="py-2.5 px-3 w-32">Encoding</th>
              <th className="py-2.5 px-3 w-28">Format</th>
              <th className="py-2.5 px-3 w-20 text-center">Action</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800/40">
            {dicts.length === 0 ? (
              <tr>
                <td colSpan={5} className="py-8 text-center text-slate-500">
                  No dictionaries configured. Click "Add Dictionary" below.
                </td>
              </tr>
            ) : (
              dicts.map((dict, i) => {
                const isUrl = dict.url !== undefined;
                const value = isUrl ? dict.url || "" : dict.path || "";
                const isRefreshing = refreshingIdx === i;

                return (
                  <tr key={i} className="hover:bg-slate-900/50 transition-colors">
                    {/* Source Type Selector */}
                    <td className="py-2 px-3 align-middle">
                      <Select
                        value={isUrl ? "Url" : "File"}
                        onValueChange={(val: "File" | "Url") => handleTypeChange(i, val)}
                      >
                        <SelectTrigger className="h-8 text-xs bg-slate-900/80 border-slate-700/60">
                          <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value="File">
                            <span className="flex items-center gap-1.5">
                              <FileText className="w-3.5 h-3.5 text-indigo-400" />
                              Local File
                            </span>
                          </SelectItem>
                          <SelectItem value="Url">
                            <span className="flex items-center gap-1.5">
                              <Globe className="w-3.5 h-3.5 text-sky-400" />
                              Online URL
                            </span>
                          </SelectItem>
                        </SelectContent>
                      </Select>
                    </td>

                    {/* Path / URL Input & Action */}
                    <td className="py-2 px-3 align-middle">
                      <div className="flex items-center gap-1.5">
                        <Input
                          type="text"
                          value={value}
                          onChange={(e) => handlePathOrUrlChange(i, e.target.value)}
                          placeholder={isUrl ? "https://example.com/SKK-JISYO.L" : "/path/to/dict"}
                          className="h-8 text-xs font-mono bg-slate-900/80 border-slate-700/60 flex-1"
                        />

                        {isUrl ? (
                          <Button
                            type="button"
                            variant="secondary"
                            size="sm"
                            disabled={isRefreshing || !value}
                            onClick={() => handleRefreshCache(i, value)}
                            title="Force Refresh Cache"
                            className="h-8 px-2.5 text-xs text-sky-400 hover:text-sky-300"
                          >
                            {isRefreshing ? (
                              <Loader2 className="w-3.5 h-3.5 animate-spin" />
                            ) : (
                              <RefreshCw className="w-3.5 h-3.5" />
                            )}
                          </Button>
                        ) : (
                          <Button
                            type="button"
                            variant="secondary"
                            size="sm"
                            onClick={() => handleBrowseFile(i)}
                            title="Browse File"
                            className="h-8 px-2.5 text-xs text-slate-300 hover:text-white"
                          >
                            <FolderOpen className="w-3.5 h-3.5" />
                          </Button>
                        )}
                      </div>
                    </td>

                    {/* Encoding Selector */}
                    <td className="py-2 px-3 align-middle">
                      <Select
                        value={dict.encoding}
                        onValueChange={(val: Encoding) => handleEncodingChange(i, val)}
                      >
                        <SelectTrigger className="h-8 text-xs bg-slate-900/80 border-slate-700/60">
                          <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value="Utf8">UTF-8</SelectItem>
                          <SelectItem value="Eucjp">EUC-JP</SelectItem>
                        </SelectContent>
                      </Select>
                    </td>

                    {/* Format Selector */}
                    <td className="py-2 px-3 align-middle">
                      <Select
                        value={dict.format}
                        onValueChange={(val: DictFormat) => handleFormatChange(i, val)}
                      >
                        <SelectTrigger className="h-8 text-xs bg-slate-900/80 border-slate-700/60">
                          <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value="Skk">SKK</SelectItem>
                          <SelectItem value="Mozc">Mozc</SelectItem>
                        </SelectContent>
                      </Select>
                    </td>

                    {/* Delete button */}
                    <td className="py-2 px-3 align-middle text-center">
                      <Button
                        type="button"
                        variant="ghost"
                        size="sm"
                        onClick={() => handleDelete(i)}
                        title="Remove Dictionary"
                        className="h-8 w-8 p-0 text-slate-400 hover:text-rose-400 hover:bg-rose-950/20"
                      >
                        <Trash2 className="w-3.5 h-3.5" />
                      </Button>
                    </td>
                  </tr>
                );
              })
            )}
          </tbody>
        </table>
      </div>

      <Button
        type="button"
        variant="outline"
        size="sm"
        onClick={handleAdd}
        className="gap-1.5 text-xs border-dashed border-slate-700/80 hover:border-indigo-500/80 hover:bg-indigo-950/20 text-slate-300"
      >
        <Plus className="w-3.5 h-3.5 text-indigo-400" />
        Add Dictionary
      </Button>
    </div>
  );
};
