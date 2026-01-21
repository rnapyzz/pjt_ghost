import { useJobs } from "@/features/jobs/hooks/useJobs";
import { useProjects } from "@/features/projects/hooks/useProjects";
import { useServices } from "@/features/services/hooks/useServices";
import { Loader2 } from "lucide-react";

export function MatrixPage() {
  const { data: services, isLoading: isLoadingServices } = useServices();
  const { data: projects, isLoading: isLoadingProjects } = useProjects();
  const { data: jobs, isLoading: isLoadingJobs } = useJobs();

  const isLoading = isLoadingServices || isLoadingProjects || isLoadingJobs;

  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <Loader2 className="w-8 h-8 animate-spin text-slate-400" />
      </div>
    );
  }

  if (!services || !projects || !jobs) {
    return <div>Failed to load data.</div>;
  }

  return (
    <div className="flex f-hull flex-col">
      <div className="mb-6 flex items-center justify-between">
        {/* header */}
        <div>
          <h1 className="text-2xl font-bold text-slate-900 tracking-tight">
            Matrix View
          </h1>
          <p className="text-slate-500 text-sm">Service * Project</p>
        </div>
      </div>

      {/* matrix table */}
      <div>
        <table>
          <thead>
            <tr>
              {/* 左上の空白セル */}
              <th>Service \ Project</th>
              {/* プロジェクトあり: Projectの数だけ列を追加 */}
              {projects?.map((project) => (
                <th>{project.name}</th>
              ))}
              {/* プロジェクトなし */}
              <th>(No Project)</th>
            </tr>
          </thead>

          <tbody>
            {/* サービスの数だけ行を追加、プロジェクト*サービスに該当するJobを各列に追加 */}
            {services?.map((service) => (
              <tr>
                {/* 行ヘッダー */}
                <th>{service.name}</th>

                {/* データセル */}
                {projects?.map((project) => (
                  <td>x</td>
                ))}

                {/* プロジェクトなし */}
                <td className="text-xs text-slate-300">no project job</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
