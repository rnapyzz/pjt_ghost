import { AlertCircle } from "lucide-react";
import { Link } from "react-router-dom";

export function NotFoundPage() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-slate-50 p-4 text-center">
      <div className="mb-4 rounded-full bg-slate-200 p-4">
        <AlertCircle className="h-10 w-10 text-slate-500" />
      </div>
      <h1 className="mb-2 text-4xl font-bold text-slate-900">404</h1>
      <h2 className="mb-4 text-xl font-semibold text-slate-700">
        Page Not Found
      </h2>
      <p className="mb-8 max-w-md text-slate-500">
        The page you are looing for doesn't exist or has been moved.
      </p>
      <Link
        to="/"
        className="rounded-md bg-slate-900 px-6 py-2.5 text-sm font-medium text-white transition-colors hover:bg-slate-800 focus:outline-none focus:ring-2 focus:ring-slate-400 focus:ring-offset-2"
      >
        Go back home
      </Link>
    </div>
  );
}
