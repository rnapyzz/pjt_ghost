import { ServiceCard } from "@/features/services/components/ServiceCard";
import { ServiceTable } from "@/features/services/components/ServiceTable";
import { useServices } from "@/features/services/hooks/useServices";
import { LayoutGrid, List, Plus } from "lucide-react";
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
      <div className="">
        {/* search */}
        <div>検索機能</div>
        {/* view toggle button */}
        <div>
          <button>
            <LayoutGrid />
          </button>
          <button>
            <List />
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
                  service={service}
                  onClick={function (slug: string): void {
                    throw new Error("Function not implemented.");
                  }}
                />
              );
            })}
          </div>
        ) : (
          // table view
          <ServiceTable />
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
