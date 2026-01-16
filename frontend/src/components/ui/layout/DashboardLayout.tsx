import { Link, Outlet, useNavigate } from "react-router-dom";
import { Button } from "../button";
import {
  FlagTriangleRight,
  FolderKanban,
  Gift,
  LayoutDashboard,
  LogOut,
  Pin,
  Settings,
  UserIcon,
} from "lucide-react";
import { useUser } from "@/hooks/useUser";

export function DashboardLayout() {
  const navigate = useNavigate();

  const { data: user, isLoading } = useUser();

  const handleLogout = () => {
    localStorage.removeItem("token");
    navigate("/login");
  };

  return (
    <div className="flex h-screen w-full bg-white text-slate-900">
      {/* sidebar */}
      <aside className="w-48 bg-slate-100 border-r flex flex-col p-4">
        {/* logo */}
        <div className="mb-8 pl-4">
          <h1 className="text-2xl font-bold tracking-tight">Ghost;</h1>
        </div>

        {/* navigation links */}
        <nav className="flex-1 space-y">
          {/* pinned */}
          <Button variant="ghost" className="w-full justify-start" asChild>
            <Link to="/">
              <Pin className="mr-2 h-4 w-4" />
              Pinned
            </Link>
          </Button>
          {/* dashboard */}
          <Button variant="ghost" className="w-full justify-start" asChild>
            <Link to="/">
              <LayoutDashboard className="mr-2 h-4 w-4" />
              Dashboard
            </Link>
          </Button>
          {/* themes */}
          <Button variant="ghost" className="w-full justify-start" asChild>
            <Link to="/themes">
              <FlagTriangleRight className="mr-2 h-4 w-4" />
              Themes
            </Link>
          </Button>
          {/* services */}
          <Button variant="ghost" className="w-full justify-start" asChild>
            <Link to="/services">
              <Gift className="mr-2 h-4 w-4" />
              Services
            </Link>
          </Button>
          {/* projects */}
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
            onClick={handleLogout}
          >
            <LogOut className="mr-2 h-4 w-4" />
            Logout
          </Button>

          {/* ユーザー情報の表示 */}
          <div className="mt-auto pt-6 border-t border-slate-200">
            {isLoading ? (
              <div className="h-10 animate-pulse bg-slate-200 rounded" />
            ) : user ? (
              <div className="flex items-center gap-3 px-2">
                <div className="h-10 w-10 rounded-full bg-blue-100 flex items-center justify-center text-blue-600">
                  <UserIcon className="h-6 w-6" />
                </div>
                <div className="flex-1 overflow-hidden">
                  <p className="text-sm font-medium truncate">{user.name}</p>
                  <p className="text-xs text-slate-500 truncate">
                    {user.username}
                  </p>
                </div>
              </div>
            ) : null}
          </div>
        </div>
      </aside>
      {/* main content area */}
      <main className="flex-1 overflow-auto bg-white p-8">
        <Outlet />
      </main>
    </div>
  );
}
