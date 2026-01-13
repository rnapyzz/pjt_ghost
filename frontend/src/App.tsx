import { BrowserRouter, Route, Routes } from "react-router-dom";
import { DashboardLayout } from "./components/ui/layout/DashboardLayout";
import { Home } from "./pages/Home";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<DashboardLayout />}>
          <Route path="/" element={<Home />} />
          <Route path="/projects" element={<p>This is a projects page.</p>} />
          <Route path="/settings" element={<p>This is a setting page.</p>} />
        </Route>

        <Route path="/login" element={<p>Login</p>} />
        <Route path="/signup" element={<p>Signup</p>} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
