import { useState } from "react";
import { ProjectType } from "../types";
import { useCreateProject } from "../hooks/useProjects";
import { Layers, Loader2, X } from "lucide-react";
import { useThemes } from "@/features/themes/hooks/useThemes";

type Props = {
  isOpen: boolean;
  onClose: () => void;
};

export function CreateProjectDialog({ isOpen, onClose }: Props) {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [themeId, setThemeId] = useState("");
  const [type, setType] = useState<ProjectType>(ProjectType.Normal);

  const { data: themes } = useThemes();

  const { mutate, isPending } = useCreateProject(() => {
    setName("");
    setDescription("");
    setThemeId("");
    setType(ProjectType.Normal);
    onClose();
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name) return;

    mutate({
      name,
      description: description || undefined,
      theme_id: themeId || undefined,
      type: type,
      attributes: {},
    });
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 animate-in fade-in duration-50">
      <div className="w-full max-w-lg rounded-lg bg-white p-6 shadow-xl animate-in zoom-in-95 duration-200">
        {/* header */}
        <div className="mb-4 flex items-center justify-between">
          <h2 className="text-xl font-bold text-slate-900">New Project</h2>
          <button
            onClick={onClose}
            className="text-slate-400 hover:text-slate-600"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* form */}
        <form onSubmit={handleSubmit} className="space-y-4">
          {/* theme (optional) */}
          <div>
            <label className="mb-1 block text-sm font-medium text-slate-700">
              Theme
            </label>
            <div className="relative">
              <Layers className="absolute left-3 top-2.5 h-4 w-4 terxt-slate-400" />
              <select
                value={themeId}
                onChange={(e) => setThemeId(e.target.value)}
                className="w-full rounded-md border border-slate-300 bg-white pl-9 pr-3 py-2 text-sm focus:ring-1 focus:ring-blue-500 focus:outline-none;"
              >
                <option value="">No Theme (Independent Project)</option>
                {themes?.map((t) => (
                  <option key={t.id} value={t.id}>
                    {t.title}
                  </option>
                ))}
              </select>
            </div>
          </div>

          {/* name */}
          <div>
            <label>Project Name</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full rounded-md border border-slate-300 px-3 py-2 text-sm focus:ring-1 focus:ring-blue-500 focus:outline-none"
              placeholder="e.g. Websight Redesign"
              required
            />
          </div>

          {/* project type */}
          <div>
            <label>Project Type</label>
            <select
              value={type}
              onChange={(e) => setType(e.target.value as ProjectType)}
              className="w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-sm focus:ring-1 focus:ring-blue-500 focus:outline-none"
            >
              {Object.values(ProjectType).map((t) => (
                <option key={t} value={t}>
                  {t}
                </option>
              ))}
            </select>
            <p className="mt-1 text-xs text-slate-500">
              Select the methodlogy or nature of this project.
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
              className="w-full rounded-md border border-slate-300 px-3 py-2 text-sm focus:ring-1 focus:ring-blue-500 focus:outline-none"
              rows={3}
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
              disabled={isPending || !name}
              className="flex items-center rounded-md bg-slate-900 px-4 py-2 text-white text-sm hover:bg-slate-800 disabled:opacity-50"
            >
              Create Theme
              {isPending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
