import { Link } from "react-router-dom";
import { FolderKanban } from "lucide-react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
// 作成したフックとコンポーネントをインポート
import { useProjects } from "@/features/projects/api/getProjects";
import { CreateProjectDialog } from "@/features/projects/components/CreateProjectDialog";

export const ProjectPage = () => {
  // フックを使うだけで、ローディング・エラー・データ取得を全部やってくれます
  const { data: projects, isLoading, isError, error } = useProjects();

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
        {/* Propsを渡す必要なし！置くだけで動きます */}
        <CreateProjectDialog />
      </div>

      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        {projects?.length === 0 ? (
          <div className="col-span-full flex flex-col items-center justify-center py-12 text-muted-foreground border-2 border-dashed rounded-lg bg-card">
            <FolderKanban size={48} className="mb-4 opacity-50" />
            <p className="text-lg font-medium">No projects found</p>
            <p className="text-sm text-muted-foreground mb-4">
              Get started by creating your first project.
            </p>
            {/* データがない時用の作成ボタン（中身はダイアログと同じトリガー） */}
            <CreateProjectDialog />
          </div>
        ) : (
          projects?.map((project) => (
            <Link
              key={project.id}
              to={`/projects/${project.id}`}
              className="block h-full group"
            >
              <Card className="h-full flex flex-col hover:shadow-lg transition-all duration-200 border-muted group-hover:border-primary/50">
                <CardHeader>
                  <CardTitle className="flex justify-between items-start text-xl">
                    <span
                      className="line-clamp-1 group-hover:text-primary transition-colors"
                      title={project.name}
                    >
                      {project.name}
                    </span>
                  </CardTitle>
                  <CardDescription>
                    Updated {new Date(project.updated_at).toLocaleDateString()}
                  </CardDescription>
                </CardHeader>
                <CardContent className="flex-1">
                  <p className="text-sm text-muted-foreground whitespace-pre-wrap line-clamp-3">
                    {project.description || "No description provided."}
                  </p>
                </CardContent>
              </Card>
            </Link>
          ))
        )}
      </div>
    </div>
  );
};
