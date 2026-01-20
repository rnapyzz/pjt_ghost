import { useProjects } from "@/features/projects/hooks/useProjects";
import { useServices } from "@/features/services/hooks/useServices";
import { useThemes } from "@/features/themes/hooks/useThemes";
import { useState } from "react";
import { useCreateJob } from "../hooks/useJobs";
import { Briefcase, Layers, Target, X } from "lucide-react";

export type Props = {
  isOpen: boolean;
  onClose: () => void;
};

export function CreateJobDialog({ isOpen, onClose }: Props) {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");

  const [serviceId, setServiceId] = useState("");
  const [projectId, setProjectId] = useState("");
  const [themeId, setThemeId] = useState("");

  const { data: services } = useServices();
  const { data: projects } = useProjects();
  const { data: themes } = useThemes();

  const { mutate, isPending } = useCreateJob(() => {
    // Reset & Close
    setTitle("");
    setDescription("");
    setServiceId("");
    setProjectId("");
    setThemeId("");
    onClose();
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!title || !serviceId) return;

    mutate({
      title,
      description: description || undefined,
      service_id: serviceId,
      project_id: projectId || undefined,
      theme_id: themeId || undefined,
    });
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 animate-in fade-in duration-50">
      <div className="w-full max-w-2xl rounded-lg bg-white p-6 shadow-xl animate-in zoom-in-95 duration-200 h-[90vh] overflow-y-auto">
        {/*  */}
        <div className="mb-6 flex items-center justify-between">
          <div>
            <h2 className="text-xl font-bold text-slate-900">New Job</h2>
            <p className="text-sm text-slate-500">
              Define a new task or responsibilities.
            </p>
          </div>
          <button
            onClick={onClose}
            className="text-slate-400 hover:text-slate-600"
          >
            <X className="w-6 h-6" />
          </button>
        </div>
        {/* form */}
        <form onSubmit={handleSubmit} className="space-y-6">
          {/* title & description */}
          <div className="space-y-4 rounded-lg bg-slate-50 p-4 border border-slate-100">
            {/* title */}
            <div>
              <label className="mg-1 block text-sm font-medium text-slate-700">
                Job Title <span className="text-red-500">*</span>
              </label>
              <input
                type="text"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                className="w-full rounded-md border border-slate-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                placeholder="e.g. Data analize Reporting"
                required
              />
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
                rows={2}
              />
            </div>
          </div>

          {/* vertical axis (service) - Required */}
          <div>
            <label className="mb-2 flex items-center text-sm font-medium text-slate-700">
              <Briefcase className="mr-2 h-4 w-4 text-slate-500" />
              Service <span className="text-red-500">*</span>
            </label>
            <select
              value={serviceId}
              onChange={(e) => setServiceId(e.target.value)}
              className="w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-sm focus:ring-1 focus:ring-blue-500"
              required
            >
              <option value="" disabled>
                Select a service...
              </option>
              {services?.map((s) => (
                <option key={s.id} value={s.id}>
                  {s.name}
                </option>
              ))}
            </select>
            <p className="mt-1 text-xs text-slate-500">
              Which service/product pays for this job?
            </p>
          </div>

          {/* horizontal axis (project or theme) - option */}
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
            {/* project */}
            <div>
              <label
                className={`mb-2 flex items-center text-sm font-medium ${projectId ? "text-slate-400" : "text-slate-700"}`}
              >
                <Target className="mr-2 h-4 w-4 text-slate-500" />
                Project (Optional)
              </label>
              <select
                value={projectId}
                onChange={(e) => {
                  const newProjectId = e.target.value;
                  setProjectId(newProjectId);
                  if (newProjectId) {
                    setThemeId("");
                  }
                }}
                className="w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-smfocus:ring-1 focus:ring-blue-500 disabled:bg-slate-100 disabled:text-slate-400"
              >
                <option value="">No Project</option>
                {projects?.map((p) => (
                  <option key={p.id} value={p.id}>
                    {p.name}
                  </option>
                ))}
              </select>
            </div>

            {/* theme */}
            <div>
              <label
                className={`mb-2 flex items-center text-sm font-medium ${projectId ? "text-slate-400" : "text-slate-700"}`}
              >
                <Layers className="mr-2 h-4 w-4 text-slate-500" />
                Theme (Optional)
              </label>
              <select
                value={themeId}
                onChange={(e) => setThemeId(e.target.value)}
                disabled={!!projectId}
                className="w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-sm focus:ring-1 focus:ring-blue-500 disabled:bg-slate-100 disabled:text-slate-400"
              >
                <option value="">No Theme</option>
                {themes?.map((t) => (
                  <option key={t.id} value={t.id}>
                    {t.title}
                  </option>
                ))}
              </select>
              {projectId && (
                <p className="mt-1 text-xs test-orange-500">
                  Disabled because a Project is selected.
                </p>
              )}
            </div>
          </div>

          {/* button */}
          <div className="flex justify-end gap-2 border-t pt-4">
            <button
              type="button"
              onClick={onClose}
              className="rounded-md px-4 py-2 text-sm font-medium text-slate-700 hover:bg-slate-100"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={isPending || !title || !serviceId}
              className="flex items-center rounded-md bg-slate-900 px-6 py-2 text-white text-sm font-medium hover:bg-slate-800 disabled:opacity-50"
            >
              Create Job
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
