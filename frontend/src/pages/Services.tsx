import { Plus } from "lucide-react";

export function Services() {
  return (
    <div>
      {/* header */}
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        {/* page title */}
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">
            Services
          </h1>
          <p className="text-sm text-slate-500">Manage your services.</p>
        </div>
        {/* create button */}
        <button
          onClick={() => console.log("not implemented")}
          className="inline-flex items-center justify-center rounded-md bg-slate-900 px-4 py-2 text-sm font-mediunm text-white shadow hover:bg-slate-800 transition-colors"
        >
          <Plus className="mr-2 h-4 w-4" />
          New Service
        </button>
      </div>

      {/* toolbar */}
      <div></div>

      {/* view toggle button */}
      <div></div>

      {/* content area */}
      <div></div>

      {/* dialog */}
      <div></div>
    </div>
  );
}
