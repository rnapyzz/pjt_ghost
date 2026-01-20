import { useState } from "react";
import { useCreateSegment } from "../hooks/useSegments";
import { Loader2, X } from "lucide-react";

type Props = {
  isOpen: boolean;
  onClose: () => void;
};

export function CreateSegmentDialog({ isOpen, onClose }: Props) {
  const [name, setName] = useState("");
  const [slug, setSlug] = useState("");
  const [description, setDescription] = useState("");

  const { mutate, isPending } = useCreateSegment(() => {
    setName("");
    setSlug("");
    setDescription("");
    onClose();
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name) return;

    mutate({
      name,
      slug: slug,
      description: description || undefined,
    });
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 animate-in fade-in duration-50">
      <div className="w-full max-w-lg rounded-lg bg-white p-6 shadow-xl animate-in zoom-in-95 duration-200">
        {/* header */}
        <div className="mb-4 flex items-center justify-between">
          <h2 className="text-xl  font-bold text-slate-900">New Segment</h2>
          <button
            onClick={onClose}
            className="text-slate-400 hover:text-slate-600"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* form */}
        <form onSubmit={handleSubmit} className="space-y-4">
          {/* name */}
          <div>
            <label className="mb-1 block text-sm font-medium text-slate-700">
              Segment Name <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full rounded-md border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              placeholder="e.g. Media Marketing Business"
              required
            />
          </div>
          {/* slug */}
          <div>
            <label className="mb-1 block text-sm font-medium text-slate-700">
              Slug <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              value={slug}
              onChange={(e) => setSlug(e.target.value)}
              className="w-full rounded-md border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              placeholder="e.g. media-and-marketing"
            />
            <p className="mt-1 text-xs text-slate-500">
              Leave empty to auto-generate.
            </p>
          </div>
          {/* description */}
          <div>
            <label className="mb-1 block text-sm font-medium text-slate-700">
              Description
            </label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="w-full rounded-md border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              rows={3}
              placeholder="Wahat is this segment about ?"
            />
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
              disabled={isPending || !name || !slug}
              className="flex items-center rounded-md bg-slate-900 px-4 py-2 text-white text-sm font-medium hover:bg-slate-800 disabled:opacity-40"
            >
              {isPending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              Create Segment
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
