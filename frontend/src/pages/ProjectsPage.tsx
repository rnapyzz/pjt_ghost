import { CreateProjectDialog } from "@/features/projects/components/CreateProjectDialog";
import { useProjects } from "@/features/projects/hooks/useProjects";
import { ProjectType } from "@/features/projects/types";
import { useThemes } from "@/features/themes/hooks/useThemes";
import { Calendar, FolderGit2, Loader2, Plus, Target } from "lucide-react";
import { useState } from "react";

export function ProjectsPage() {
  const { data: projects, isLoading } = useProjects();
  const { data: themes } = useThemes();
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  const getThemeName = (id?: string) => {
    if (!id) return null;
    return themes?.find((t) => t.id === id)?.title;
  };

  const getTypeBadgeColor = (type: string) => {
    switch (type) {
      case ProjectType.Agile:
        return "bg-blue-100 text-blue-700";
      case ProjectType.Maintenance:
        return "bg-orange-100 text-orange-700";
      case ProjectType.RandD:
        return "bg-purple-100 text-purple-700";
      default:
        return "bg-slate-100 text-slate-700";
    }
  };

  if (isLoading) {
    return (
      <div className="p-8 flex justify-center">
        <Loader2 className="animate-spin" />
      </div>
    );
  }

  return (
    <div className="mb-8 max-w-7xl mx-auto">
      <div className="mb-8 flex items-center justify-between">
        {/* header */}
        <div>
          <h1 className="text-2xl font-bold text-slate-900 tracking-tight">
            Projects
          </h1>
          <p className="text-sm text-slate-500">
            Track initiatives and development efforts.
          </p>
        </div>
        <button
          onClick={() => setIsDialogOpen(true)}
          className="flex items-center rounded-md bg-slate-900 px-4 py-2 text-sm font-medium text-white hover:bg-slate-800"
        >
          <Plus className="mr-2 h-4 w-4" />
          New Project
        </button>
      </div>

      {/* project list */}
      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
        {projects?.map((project) => (
          <div
            key={project.id}
            className="group flex flex-col justify-between rounded-lg border border-slate-200 bg-white p-6 shadow-sm transition-all hover:shadow-md"
          >
            <div>
              <div className="mb-4 flex items-center justify-between">
                {/* type badge */}
                <span
                  className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${getTypeBadgeColor(project.project_type as string)}`}
                >
                  {project.project_type}
                </span>
                {/* active status */}
                <div
                  className={`h-2.5 w-2.5 rounded-full ${project.is_active ? "bg-green-500" : "bg-slate-300"}`}
                ></div>
              </div>

              {/* name */}
              <div className="flex items-center gap-2">
                <FolderGit2 className="h-5 w-5 text-slate-400" />
                <h3>{project.name}</h3>
              </div>

              {/* theme link */}
              {project.theme_id && (
                <div className="mt-2 flex items-center text-xs text-purple-400 font-meduim">
                  <Target className="mr-1 h-3 w-3" />
                  {getThemeName(project.theme_id)}
                </div>
              )}

              {/* description */}
              <p className="mt-3 text-xs text-slate-500 line-clamp-3">
                {project.description || "No description provided."}
              </p>
            </div>
            <div className="mt-4 flex  items-center justify-between border-t border-slate-100 text-xs text-slate-500">
              <div className="flex items-center">
                <Calendar className="mr-1 h-3 w-3" />
                {new Date(project.created_at).toLocaleDateString()}
              </div>
              <span className="cursor-pointer font-medium text-blue-600 hover:underline">
                View Tasks &rarr;
              </span>
            </div>
          </div>
        ))}

        {projects?.length === 0 && <p>no project</p>}
      </div>

      {/* dialog */}
      <CreateProjectDialog
        isOpen={isDialogOpen}
        onClose={() => setIsDialogOpen(false)}
      />
    </div>
  );
}
