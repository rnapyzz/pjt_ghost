import { useJobs } from "@/features/jobs/hooks/useJobs";
import { useProjects } from "@/features/projects/hooks/useProjects";
import { useServices } from "@/features/services/hooks/useServices";
import { Loader2, Plus } from "lucide-react";

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

  const getJobsForCell = (serviceId: string, projectId: string) => {
    return jobs.filter(
      (job) => job.service_id === serviceId && job.project_id === projectId,
    );
  };

  const getNoProjectJobs = (serviceId: string) => {
    return jobs.filter(
      (job) => job.service_id === serviceId && !job.project_id,
    );
  };

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
      <div className="flex-1 overflow-auto border rounded-lg bg-white shadow-sm relative">
        <table className="min-w-max border-collapse">
          <thead className="bg-slate-50 sticky top-0 z-20 shadow-sm">
            <tr>
              {/* 左上の空白セル */}
              <th className="sticky left-0 top-0 z-30 bg-slate-50 p-4 min-w-50 border-b border-r border-slate-200 text-left text-xs font-semibold uppercase text-slate-500">
                Service \ Project
              </th>

              {/* プロジェクトあり: Projectの数だけ列を追加 */}
              {projects.map((project) => (
                <th
                  key={project.id}
                  className="p-4 min-50 border-b border-slate-200 text-left text-sm font-semibold text-slate-900"
                >
                  <div className="flex items-center gap-2">
                    <span
                      className={`h-2 w-2 rounded-full ${project.is_active ? "bg-green-400" : "bg-slate-400"}`}
                    />
                    {project.name}
                  </div>
                </th>
              ))}
              {/* プロジェクトなし */}
              <th className="p-4 min-w-50 border-b border-slate-100 text-left text-sm font-semibold text-slate-400 bg-slate-50/50">
                (No Project)
              </th>
            </tr>
          </thead>

          <tbody className="divide-y divide-slate-200">
            {/* サービスの数だけ行を追加、プロジェクト*サービスに該当するJobを各列に追加 */}
            {services.map((service) => (
              <tr key={service.id}>
                {/* 行ヘッダー */}
                <th className="sticky left-0 z-10 p-4 bg-white border-r border-slate-200 text-left align-top">
                  <div className="font-medium text-slate-900">
                    {service.name}
                  </div>
                  <div className="text-xs text-slate-300 font-normal mt-1">
                    #{service.slug}
                  </div>
                </th>

                {/* データセル */}
                {projects.map((project) => {
                  const jobsForCell = getJobsForCell(service.id, project.id);
                  return (
                    <td
                      key={project.id}
                      className="p-3 align-top border-r border-slate-100 hover:bg-slate-50 transition-colors"
                    >
                      <div className="space-y-2">
                        {jobsForCell.map((job) => (
                          <div
                            key={job.id}
                            className="group relative rounded border border-slate-200 bg-white p-2 shadow-sm hover:border-blue-300 hover:shadow-md transition-all text-slate-900 hover:text-blue-500"
                          >
                            <div className="text-sm font-medium">
                              {job.title}
                            </div>
                            <div className="mt-1 flex items-center justify-between">
                              <span className="text-[10px] bg-slate-100 px-1.5 py-0.5 rounded text-slate-600">
                                {job.status}
                              </span>
                            </div>
                          </div>
                        ))}

                        {jobsForCell.length === 0 && (
                          <div className="h-full min-h-10 flex items-center justify-center opacity-0 hover:opacity-100 transition-opacity">
                            <button className="text-slate-400 hover:text-blue-600">
                              <Plus className="h-5 w-5" />
                            </button>
                          </div>
                        )}
                      </div>
                    </td>
                  );
                })}

                {/* プロジェクトなし */}
                <td className="p-3 align-top bg-slate-50/30">
                  <div className="space-y-2">
                    {getNoProjectJobs(service.id).map((job) => (
                      <div
                        key={job.id}
                        className="rounded border border-dashed border-slate-300 bg-slate-50 hover:bg-slate-100 p-2 text-slate-500 hover:text-blue-500"
                      >
                        <div className="text-sm">{job.title}</div>
                      </div>
                    ))}
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
