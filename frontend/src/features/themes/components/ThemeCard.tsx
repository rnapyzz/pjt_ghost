import { Clock } from "lucide-react";
import type { Theme } from "../types";

type Props = {
  theme: Theme;
  onClick: (id: string) => void;
};

export function ThemeCard({ theme, onClick }: Props) {
  return (
    <div
      onClick={() => onClick(theme.id)}
      className="group relative flex flex-col justify-between rounded-xl border border-slate-200 bg-white py-2 px-4 shadow-xs transition-all hover:shadow-md hover:border-slate-300 cursor-pointer"
    >
      {/* ステータスバッジ */}
      <div className="absolute top-2 right-4">
        <span
          className={`inline-flex items-center rounded-lg px-3 py-0.5 text-xs font-medium ${
            theme.is_active
              ? "bg-green-100 text-green-800"
              : "bg-slate-100 text-slate-600"
          }`}
        >
          {theme.is_active ? "Open" : "Closed"}
        </span>
      </div>

      {/* メイン部分 */}
      <div className="mb-4 pr-16">
        <h3 className="text-lg font-semibold text-slate-900 group-hover:text-blue-600 transition-colors">
          {theme.title}
        </h3>
        <p className="mt-2 text-sm text-slate-500 line-clamp-2">
          {theme.description}
        </p>
      </div>

      {/* フッター部分 */}
      <div className="flex items-center justify-between border-t border-slate-100 pt-4 text-xs text-slate-400">
        <div className="flex items-center gap-1">
          <Clock className="h-3 w-3" />
          <span>Updated {new Date(theme.updated_at).toLocaleDateString()}</span>
        </div>
      </div>
    </div>
  );
}
