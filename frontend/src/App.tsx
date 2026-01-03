import { BrowserRouter, Routes, Route } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeProvider } from "@/components/theme-provider";
import { AppLayout } from "@/components/layouts/AppLayout";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "./components/ui/card";
// import { Badge } from "./components/ui/badge";
// import { format } from "date-fns";
import { useProjects } from "./features/projects/api/getProjects";
import { CreateProjectDialog } from "./features/projects/components/CreateProjectDialog";

// useQueryやuseMutationを使うためにqueryClientを作成しておく
const queryClient = new QueryClient();

// ホームページに表示するコンポーネント（まだ空）
const Dashboard = () => <h2 className="text-2xl font-bold">Dashboard</h2>;

// プロジェクト一覧ページを表示するためのコンポーネント
// TODO: 別ファイルに分割する
const Projects = () => {
  const { data: projects, isLoading, isError, error } = useProjects();

  if (isLoading) {
    return <div className="text-center p-10">Loading projects...</div>;
  }

  if (isError) {
    return (
      <div className="text-red-500 p-10">
        Error: {error instanceof Error ? error.message : "Failed to fetch"}
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-3xl font-bold tracking-tight">Projects</h2>
        <CreateProjectDialog />
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {projects?.map((project) => (
          <Card key={project.id} className="hover:shadow-lg transition-shadow">
            <CardHeader>
              <CardTitle className="flex justify-between items-start">
                <span>{project.name}</span>
              </CardTitle>
              <CardDescription>
                Created at {new Date(project.created_at).toLocaleDateString()}
              </CardDescription>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-muted-foreground whitespace-pre-wrap">
                {project.description || "No description provided."}
              </p>
            </CardContent>
          </Card>
        ))}

        {projects?.length === 0 && (
          <p className="text-muted-foreground">
            No projects found. Create one!
          </p>
        )}
      </div>
    </div>
  );
};

// 依存関係を意識して、データ層、設定/テーマ層、ルーター層、ルート定義の順に包む
function App() {
  return (
    // データ層: どの場所でも使い得る、UIやURLに依存しない
    <QueryClientProvider client={queryClient}>
      {/* // テーマ層: 全ページに適用する */}
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
        {/* ルーター層: この内側で<Link>などのルーター機能を使う */}
        <BrowserRouter>
          {/* コンテンツ層: 実際の画面 */}
          {/* AppLayoutの中で<Link>やToggleThemeを使っているので、他のコンポーネントはその内側に置く */}
          <Routes>
            <Route element={<AppLayout />}>
              <Route path="/" element={<Dashboard />} />
              <Route path="/projects" element={<Projects />} />
            </Route>
          </Routes>
        </BrowserRouter>
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;
