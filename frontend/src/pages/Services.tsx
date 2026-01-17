import { ServiceCard } from "@/features/services/components/ServiceCard";
import { ServiceTable } from "@/features/services/components/ServiceTable";
import { useServices } from "@/features/services/hooks/useServices";
import { LayoutGrid, List, Plus, Search } from "lucide-react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

export function Services() {
  const navigate = useNavigate();
  // ページを開いた時にデータを取得
  const { data: services, isLoading, error } = useServices();

  // UIの状態管理
  const [searchQuery, setSearchQuery] = useState("");
  const [isCreateOpen, setIsCreateOpen] = useState(false);
  const [viewMode, setViewMode] = useState<"grid" | "table">("grid");

  const filteredServices = services?.filter(
    (s) =>
      s.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      s.slug.toLowerCase().includes(searchQuery.toLowerCase()),
  );

  if (isLoading) return <div>aaaa</div>;
  if (error) return <div>bbbb</div>;

  return (
    <div className="space-y-6">
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
      <div className="flex items-center justify-between gap-4">
        {/* search */}
        <div className="relative max-w-lg flex-1">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400" />
          <input
            type="text"
            placeholder=""
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full rounded-md border border-slate-200 pl-9 pr-4 py-2 text-sm
            focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
          />
        </div>

        {/* view toggle button */}
        <div className="flex items-center rounded-md border border-slate-200 bg-white p-1">
          {/* Grid view */}
          <button
            onClick={() => setViewMode("grid")}
            className={`rounded px-2 py-1 transition-colors ${
              viewMode === "grid"
                ? "bg-slate-100 text-slate-900"
                : "text-slate-400 hover:text-slate-600"
            }`}
          >
            <LayoutGrid className="h-4 w-4" />
          </button>
          {/* table view */}
          <button
            onClick={() => setViewMode("table")}
            className={`rounded px-2 py-1 transition-colors ${
              viewMode === "table"
                ? "bg-slate-100 text-slate-900"
                : "text-slate-400 hover:text-slate-600"
            }`}
          >
            <List className="h-4 w-4" />
          </button>
        </div>
      </div>

      {/* content area */}
      {filteredServices && filteredServices.length > 0 ? (
        viewMode == "grid" ? (
          // grid view
          <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            {filteredServices.map((service) => {
              return (
                <ServiceCard
                  key={service.id}
                  service={service}
                  onClick={(slug) => navigate(`/services/${slug}`)}
                />
              );
            })}
          </div>
        ) : (
          // table view
          <ServiceTable
            services={filteredServices}
            onClick={(slug) => navigate(`/services/${slug}`)}
          />
        )
      ) : (
        <div className="rounded-lg border border-dashed border-slate-300 p-8 text-center">
          <p className="text-slate-500">No services found.</p>
        </div>
      )}

      {/* dialog */}
      <div></div>
    </div>
  );
}
