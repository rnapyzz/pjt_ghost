import { useState } from "react";
import { FolderKanban } from "lucide-react";
import { useProjects } from "@/features/projects/api/getProjects";
import { CreateProjectDialog } from "@/features/projects/components/CreateProjectDialog";
import { ProjectCard } from "@/features/projects/components/ProjectCard";

export const ProjectPage = () => {
  const { data: projects, isLoading, isError, error } = useProjects();

  // どのプロジェクトが開いているかを管理
  const [expandedProjectId, setExpandedProjectId] = useState<string | null>(
    null
  );

  if (isLoading) {
    return (
      <div className="text-center p-10 text-muted-foreground">
        Loading projects...
      </div>
    );
  }

  if (isError) {
    return (
      <div className="text-red-500 p-10 text-center">
        Error: {error instanceof Error ? error.message : "Failed to fetch"}
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-3xl font-bold tracking-tight">Projects</h2>
          <p className="text-muted-foreground">
            管理しているプロジェクト（施策）の一覧です
          </p>
        </div>
        <CreateProjectDialog />
      </div>

      {/* Gridレイアウト */}
      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3 items-start transition-all">
        {projects?.length === 0 ? (
          <div className="col-span-full flex flex-col items-center justify-center py-12 text-muted-foreground border-2 border-dashed rounded-lg bg-card">
            <FolderKanban size={48} className="mb-4 opacity-50" />
            <p className="text-lg font-medium">No projects found</p>
            <CreateProjectDialog />
          </div>
        ) : (
          projects?.map((project) => (
            <ProjectCard
              key={project.id}
              project={project}
              // 自分が拡大対象なら true
              isExpanded={project.id === expandedProjectId}
              // クリックされたらトグルする
              onToggleExpand={() => {
                setExpandedProjectId((prev) =>
                  prev === project.id ? null : project.id
                );
              }}
            />
          ))
        )}
      </div>
    </div>
  );
};
