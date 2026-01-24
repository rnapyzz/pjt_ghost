import { BrowserRouter, Route, Routes } from "react-router-dom";
import { DashboardLayout } from "./components/ui/layout/DashboardLayout";
import { Home } from "./pages/Home";
import { SignupPage } from "./pages/SingupPage";
import { LoginPage } from "./pages/LoginPage";
import { ProtectedRoute } from "./features/auth/components/ProtectedRoute";
import { ThemesPage } from "./pages/ThemesPage";
import { ServicesPage } from "./pages/ServicesPage";
import { SegmentsPage } from "./pages/SegmentsPage";
import { JobsPage } from "./pages/JobsPage";
import { ProjectsPage } from "./pages/ProjectsPage";
import { MatrixPage } from "./pages/MatrixPage";
import { NotFoundPage } from "./pages/NotFoundPage";
import { JobDetailPage } from "./pages/JobDetailPage";
import { ProjectPlanPage } from "./features/projects/pages/ProjectPlanPage";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* 公開ルート */}
        <Route path="/signup" element={<SignupPage />} />
        <Route path="/login" element={<LoginPage />} />

        {/* 保護ルート */}
        <Route element={<ProtectedRoute />}>
          <Route element={<DashboardLayout />}>
            <Route index element={<MatrixPage />} />
            <Route path="/" element={<Home />} />
            <Route path="/themes" element={<ThemesPage />} />
            <Route path="/projects" element={<ProjectsPage />} />
            <Route
              path="/projects/:projectId/plan"
              element={<ProjectPlanPage />}
            />
            <Route path="/segments" element={<SegmentsPage />} />
            <Route path="/services" element={<ServicesPage />} />
            <Route path="/jobs" element={<JobsPage />} />
            <Route path="/jobs/:jobId" element={<JobDetailPage />} />
            <Route path="/settings" element={<></>} />
          </Route>
        </Route>

        {/* catch-all route */}
        <Route path="*" element={<NotFoundPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
