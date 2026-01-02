import { Outlet, Link, useLocation } from "react-router-dom";
import {
  LayoutDashboard,
  FolderKanban,
  Settings,
  Moon,
  Sun,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { useTheme } from "@/components/theme-provider";
import { Separator } from "@/components/ui/separator";

export function AppLayout() {
  const { pathname } = useLocation();
  const { theme, setTheme } = useTheme();

  // ナビゲーションメニューの定義
  const navItems = [
    { label: "Dashboard", href: "/", icon: <LayoutDashboard size={20} /> },
    { label: "Projects", href: "/projects", icon: <FolderKanban size={20} /> },
    { label: "Settings", href: "/settings", icon: <Settings size={20} /> },
  ];

  return (
    <div className="flex h-screen w-full bg-background text-foreground">
      {/* 左サイドバー */}
      <aside className="w-64 border-r bg-muted/40 flex flex-col">
        <div className="p-6">
          <h1 className="text-xl font-bold tracking-tight">Ghost;</h1>
        </div>

        <Separator />

        <nav className="flex-1 px-4 py-4 space-y-2">
          {navItems.map((item) => (
            <Link
              key={item.href}
              to={item.href}
              className={`flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition-colors ${
                pathname === item.href
                  ? "bg-primary text-primary-foreground" // 選択中のスタイル
                  : "hover:bg-muted text-muted-foreground hover:text-foreground" // 未選択時のスタイル
              }`}
            >
              {item.icon}
              {item.label}
            </Link>
          ))}
        </nav>

        {/* サイドバー下部（テーマ切り替えなど） */}
        <div className="p-4 border-t">
          <Button
            variant="ghost"
            size="sm"
            className="w-full justify-start gap-2"
            onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
          >
            {theme === "dark" ? <Sun size={18} /> : <Moon size={18} />}
            <span>Switch Theme</span>
          </Button>
        </div>
      </aside>

      {/* メインコンテンツエリア */}
      <main className="flex-1 overflow-auto">
        <div className="p-8 max-w-6xl mx-auto">
          {/* ここに各ページの中身が差し込まれる */}
          <Outlet />
        </div>
      </main>
    </div>
  );
}
