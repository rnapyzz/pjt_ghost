// src/routes/ProjectDetailPage.tsx
import { Link, useParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { getJobs } from "@/features/jobs/api/getJobs";
import { getProject } from "@/features/projects/api/getProject";
import { CreateJobDialog } from "@/features/jobs/components/CreateJobDialog";

export const ProjectDetailPage = () => {
  const { projectId } = useParams();

  // 1. プロジェクト情報の取得
  const {
    data: project,
    isLoading: isLoadingProject,
    isError: isErrorProject,
    error: errorProject,
  } = useQuery({
    queryKey: ["project", projectId],
    queryFn: () => getProject(projectId!),
    enabled: !!projectId, // projectIdがある時だけ実行
  });

  // 2. Job一覧の取得
  const { data: jobs, isLoading: isLoadingJobs } = useQuery({
    queryKey: ["jobs", projectId],
    queryFn: () => getJobs(projectId!),
    enabled: !!projectId,
  });

  if (isLoadingProject || isLoadingJobs) {
    return <div className="p-10 text-center">Loading...</div>;
  }

  if (isErrorProject) {
    return (
      <div className="p-10 text-center text-red-500">
        <h2 className="text-xl font-bold">Error fetching project</h2>
        <p>
          {errorProject instanceof Error
            ? errorProject.message
            : "Unknown error"}
        </p>
        <p className="text-sm text-gray-500 mt-2">
          ブラウザのコンソール(F12)も確認してください
        </p>
      </div>
    );
  }

  if (!project) {
    return <div className="p-10 text-center">Project not found</div>;
  }

  return (
    <div className="space-y-6">
      {/* ヘッダー部分: プロジェクト情報 */}
      <div>
        <h2 className="text-3xl font-bold tracking-tight">{project.name}</h2>
        <p className="text-muted-foreground mt-1">
          {project.description || "No description"}
        </p>
      </div>

      <div className="border-t pt-6">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-xl font-semibold">Jobs (案件一覧)</h3>
          <CreateJobDialog projectId={projectId!} />
        </div>

        {/* Job一覧グリッド */}
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {jobs?.map((job) => (
            <Link
              key={job.id}
              to={`/projects/${projectId}/jobs/${job.id}`}
              className="block h-full"
            >
              <Card className="hover:shadow-lg transition-shadow h-full cursor-pointer">
                <CardHeader>
                  <CardTitle className="flex justify-between items-center text-lg">
                    {job.name}
                    {/* ビジネスモデルをバッジで表示 */}
                    <Badge variant="secondary">{job.business_model}</Badge>
                  </CardTitle>
                  <CardDescription>
                    {new Date(job.created_at).toLocaleDateString()}
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <p className="text-sm text-muted-foreground">
                    クリックして予実を入力
                  </p>
                </CardContent>
              </Card>
            </Link>
          ))}

          {jobs?.length === 0 && (
            <div className="col-span-full text-center py-10 text-muted-foreground bg-muted/20 rounded-lg border-dashed border-2">
              No jobs found.
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
