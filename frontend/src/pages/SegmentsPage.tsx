import { CreateSegmentDialog } from "@/features/segments/components/CreateSegmentDialog";
import { useSegments } from "@/features/segments/hooks/useSegments";
import { Layers, Loader2, Plus } from "lucide-react";
import { useState } from "react";

export function SegmentsPage() {
  const { data: segments, isLoading, error } = useSegments();
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  if (isLoading) {
    return (
      <div className="">
        <Loader2 className="h-8 w-8 animate-spin text-slate-400" />
      </div>
    );
  }

  if (error) {
    return <div className="text-red-500">Failed to load segments.</div>;
  }

  return (
    <div className="mb-8 max-w-7xl mx-auto">
      {/* header */}
      <div className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-slate-900">Segments</h1>
          <p className="text-slate-500">
            Manage your business units and domains.
          </p>
        </div>
        <button
          onClick={() => setIsDialogOpen(true)}
          className="flex items-center rounded-md bg-slate-900 px-4 py-2 text-sm font-medium text-white hover:bg-slate-800"
        >
          <Plus className="mr-2 h-4 w-4" />
          New Segment
        </button>
      </div>

      {/* grid list */}
      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2 2xl:grid-cols-3">
        {segments?.map((segment) => (
          <div
            key={segment.id}
            className="group relative flex flex-col justify-between rounded-lg border border-slate-200 bg-white p-6 shadow-sm transition-all hover:shadow-md"
          >
            <div>
              <div className="mb-4 flex h-10 w-10 items-center justify-center rounded-lg bg-blue-50 text-blue-600">
                {/* ui_config の設定に応じたアイコンを表示 */}
                <Layers className="w-6 h-6" />
              </div>
              <h3 className="text-lg font-semibold text-slate-900">
                {segment.name}
              </h3>
              <p className="mt-2 text-sm text-slate-500 line-clamp-2">
                {segment.description || "No description provided."}
              </p>
            </div>

            <div className="mt-4 pt-4 border-t border-slate-100 flex items-center justify-between text-sm text-slate-500">
              <span>{new Date(segment.created_at).toLocaleDateString()}</span>
              {/* TODO: Manage Services などのリンクを置く */}
              <span className="text-blue-600 font-medium group-hover:underline cursor-pointer">
                View Details &rarr;
              </span>
            </div>
          </div>
        ))}

        {/* Empty State */}
        {segments?.length === 0 && (
          <div className="col-span-full flex flex-col items-center justify-center round-lg border-2 border-dashed border-slate-300 py-12 text-center">
            <Layers className="max-suto h-12 w-12 text-slate-400" />
            <h3 className="mt-2 text-sm font-semibold text-slate-900">
              No segments.
            </h3>
            <p className="mt-1 text-sm text-slate-500">
              Get started by creating a new segment.
            </p>
            <div className="mt-6">
              <button
                onClick={() => setIsDialogOpen(true)}
                className="inline-flex items-center rounded-md bg-blue-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500"
              >
                <Plus className="-m-0.5 mr-1.5 h-5 w-5" aria-hidden="true" />
                New Segment
              </button>
            </div>
          </div>
        )}
      </div>

      {/* dialog */}
      <CreateSegmentDialog
        isOpen={isDialogOpen}
        onClose={() => setIsDialogOpen(false)}
      />
    </div>
  );
}
