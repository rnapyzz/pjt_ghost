import { BrowserRouter, Routes, Route } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeProvider } from "@/components/theme-provider";
import { AppLayout } from "@/components/layouts/AppLayout";
import { DashboardPage } from "./routes/DashboardPage";
import { ProjectPage } from "./routes/ProjectsPage";
import { ProjectDetailPage } from "./routes/ProjectDetailPage";

// useQueryやuseMutationを使うためにqueryClientを作成しておく
const queryClient = new QueryClient();

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
              <Route path="/" element={<DashboardPage />} />
              <Route path="/projects" element={<ProjectPage />} />
              <Route
                path="/projects/:projectId"
                element={<ProjectDetailPage />}
              />
            </Route>
          </Routes>
        </BrowserRouter>
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;
