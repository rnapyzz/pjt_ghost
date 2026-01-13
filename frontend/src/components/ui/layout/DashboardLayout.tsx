import { Link, Outlet } from "react-router-dom";
import { Button } from "../button";
import {
  FolderKanban,
  LayoutDashboard,
  LogOut,
  Pin,
  Settings,
} from "lucide-react";

export function DashboardLayout() {
  return (
    <div className="flex h-screen w-full bg-white text-slate-900">
      {/* sidebar */}
      <aside className="w-64 bg-slate-100 border-r flex flex-col p-4">
        {/* logo */}
        <div className="mb-8 pl-4">
          <h1 className="text-2xl font-bold tracking-tight">Ghost;</h1>
        </div>

        {/* navigation links */}
        <nav className="flex-1 space-y-2">
          <Button variant="ghost" className="w-full justify-start" asChild>
            <Link to="/pinned">
              <Pin className="mr-2 h-4 w-4" />
              Pinned
            </Link>
          </Button>
          <Button variant="ghost" className="w-full justify-start" asChild>
            <Link to="/">
              <LayoutDashboard className="mr-2 h-4 w-4" />
              Dashboard
            </Link>
          </Button>
          <Button variant="ghost" className="w-full justify-start" asChild>
            <Link to="/projects">
              <FolderKanban className="mr-2 h-4 w-4" />
              Projects
            </Link>
          </Button>
        </nav>

        {/* bottom section */}
        <div className="border-t pt-4 space-y-2">
          <Button variant="ghost" className="w-full justify-start" asChild>
            <Link to="/settings">
              <Settings className="mr-2 h-4 w-4" />
              Settings
            </Link>
          </Button>
          <Button
            variant="ghost"
            className="w-full justify-start hover:text-red-700 hover:bg-red-50"
          >
            <LogOut className="mr-2 h-4 w-4" />
            Logout
          </Button>
        </div>
      </aside>
      {/* main content area */}
      <main className="flex-1 overflow-auto bg-white p-8">
        <Outlet />
      </main>
    </div>
  );
}
