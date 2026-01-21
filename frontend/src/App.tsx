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

function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* 保護ルート */}
        <Route element={<ProtectedRoute />}>
          <Route element={<DashboardLayout />}>
            <Route path="/" element={<Home />} />
            <Route path="/themes" element={<ThemesPage />} />
            <Route path="/projects" element={<ProjectsPage />} />
            <Route path="/segments" element={<SegmentsPage />} />
            <Route path="/services" element={<ServicesPage />} />
            <Route path="/jobs" element={<JobsPage />} />
            <Route path="/settings" element={<></>} />
          </Route>
        </Route>

        {/* 公開ルート */}
        <Route path="/signup" element={<SignupPage />} />
        <Route path="/login" element={<LoginPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
