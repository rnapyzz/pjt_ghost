import { Input } from "@/components/ui/input";
import { ThemeCard } from "@/features/themes/components/ThemeCard";
import { useThemes } from "@/features/themes/hooks/useThemes";
import { Loader2, Plus, Search } from "lucide-react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

export function Themes() {
  const navigate = useNavigate();
  const { data: themes, isLoading, error } = useThemes();
  const [searchQuery, setSearchQuery] = useState("");
  const [isCreateOpen, setIsCreateOpen] = useState(false);

  // 検索用関数
  const filteredThemes = themes?.filter((theme) => {
    return theme.title.toLowerCase().includes(searchQuery.toLowerCase());
  });

  // Loading
  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin text-slate-400" />
      </div>
    );
  }

  // Load Error
  if (error) {
    return <div className="p-4 text-red-500">Failed to load themes</div>;
  }

  return (
    <div className="space-y-6">
      {/* Header Area */}
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">
            Themes
          </h1>
          <p className="text-sm text-slate-500">Manage your important theme.</p>
        </div>
        <button
          className="inline-flex items-center justify-center rounded-md bg-slate-900 px-4 py-2 text-sm font-medium text-white shadow hover:bg-slate-800 transition-colors"
          onClick={() => setIsCreateOpen(true)}
        >
          <Plus className="mr-2 h-4 w-4" />
          New Theme
        </button>
      </div>

      {/* Filter Area */}
      <div className="relative max-w-sm">
        <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400" />
        <Input
          type="text"
          placeholder="Search themes ..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="w-full rounded-md border border-slate-200 pl-9 pr-4 py-2 text-sm "
        />
      </div>

      {/* Grid Area */}
      {filteredThemes && filteredThemes.length > 0 ? (
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:gap-6">
          {filteredThemes.map((theme) => (
            <ThemeCard
              key={theme.id}
              theme={theme}
              onClick={(id) => navigate(`/themes/${id}`)}
            />
          ))}
        </div>
      ) : (
        <div className="rounded-lg border border-dashed border-slate-300 p-8 text-center">
          <p className="text-slate-500">No themes found.</p>
        </div>
      )}

      {/* Create Model */}
      {/* TODO */}
      <div>....</div>
    </div>
  );
}
