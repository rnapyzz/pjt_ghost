import { useState } from "react";
import { useCreateService } from "../hooks/useServices";
import { Loader2, X } from "lucide-react";
import { useSegments } from "@/features/segments/hooks/useSegments";

type Props = {
  isOpen: boolean;
  onClose: () => void;
};

export function CreateServiceDialog({ isOpen, onClose }: Props) {
  const [name, setName] = useState("");
  const [slug, setSlug] = useState("");
  const [segmentId, setSegmentId] = useState("");

  const { data: segments, isLoading: isLoadingSegments } = useSegments();

  const { mutate, isPending } = useCreateService(() => {
    setName("");
    setSlug("");
    setSegmentId("");
    onClose();
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name || !segmentId) return;
    mutate({ name, slug: slug || undefined, segment_id: segmentId });
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 animate-in fade-in duration-50">
      <div className="w-full max-w-lg rounded-lg bg-white p-6 shadow-xl animate-in zoom-in-95 duration-200">
        {/* header */}
        <div className="mb-4 flex items-center justify-between">
          <h2 className="text-xl font-bold text-slate-900">New Service</h2>
          <button
            onClick={onClose}
            className="text-slate-400 hover:text-slate-600"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          {/* segment */}
          <div>
            <label className="mb-1 block text-sm font-medium text-slate-700">
              Segment <span className="text-red-500">*</span>
            </label>
            <select
              value={segmentId}
              onChange={(e) => setSegmentId(e.target.value)}
              className="w-full appearance-none rounded-md border border-slate-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 disabled:bg-slate-100 disabled:text-slate-500"
              required
              disabled={isLoadingSegments}
            >
              <option value="" disabled>
                Select a segment...
              </option>
              {segments?.map((segment) => (
                <option key={segment.id} value={segment.id}>
                  {segment.name}
                </option>
              ))}
            </select>
            {isLoadingSegments && (
              <div>
                <Loader2 />
              </div>
            )}
          </div>
          {/* name */}
          <div>
            <label className="mb-1 block text-sm font-medium text-slate-700">
              Service Name <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full rounded-md border border-slate-300 px-3 py-2 text-sm
              focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              placeholder="e.g. Reward Media"
              required
            />
          </div>
          {/* slug */}
          <div>
            <label className="mb-1 block text-sm font-medium text-slate-700">
              Slug
            </label>
            <input
              type="text"
              value={slug}
              onChange={(e) => {
                setSlug(e.target.value);
              }}
              className="w-full rounded-md border border-slate-300 px-3 py-2 text-sm
              focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              placeholder="e.g. reward-media"
            />
            <p className="mt-1 text-xs text-slate-500">
              Leave empty to auto-generate from name.
            </p>
          </div>
          {/* button */}
          <div className="flex justify-end gap-2 pt-2">
            <button
              type="button"
              onClick={onClose}
              className="rounded-md px-4 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={isPending || !segmentId}
              className="flex items-center rounded-md bg-slate-900 px-4 py-2 text-white text-sm font-medium hover:bg-slate-800 disabled:opacity-40 disabled:bg-slate-900"
            >
              {isPending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              Create
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
