import { CreateJobDialog } from "@/features/jobs/components/CreateJobDialog";
import { useJobs } from "@/features/jobs/hooks/useJobs";
import { Briefcase, Calendar, Loader2, Plus } from "lucide-react";
import { useState } from "react";

export function JobsPage() {
  const { data: jobs, isLoading } = useJobs();
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  if (isLoading) {
    return (
      <div className="p-8 flex justify-center">
        <Loader2 className="animate-spin" />
      </div>
    );
  }

  return (
    <div className="mb-8 max-w-7xl mx-auto">
      {/* Header  */}
      <div className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-slate-900">Jobs</h1>
          <p className="text-sm text-slate-500">
            Manage all plans and responsibilities.
          </p>
        </div>
        <button
          onClick={() => setIsDialogOpen(true)}
          className="flex items-center rounded-md bg-slate-900 px-4 py-2 text-sm font-medium text-white hover:bg-slate-800"
        >
          <Plus className="mr-2 h-4 w-4" />
          New Job
        </button>
      </div>

      {/* Table Layout */}
      <div className="">
        <table className="min-w-full divide-y divide-slate-200">
          <thead className="bg-slate-50">
            <tr>
              <th className="px-6 py-3 text-xs font-medium text-slate-500 tracking-wider">
                Title
              </th>
              <th className="px-6 py-3 text-xs font-medium text-slate-500 tracking-wider">
                Status
              </th>
              <th className="px-6 py-3 text-xs font-medium text-slate-500 tracking-wider">
                Service
              </th>
              <th className="px-6 py-3 text-xs font-medium text-slate-500 tracking-wider">
                Context
              </th>
              <th className="px-6 py-3 text-xs font-medium text-slate-500 tracking-wider">
                Created
              </th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-slate-200">
            {jobs?.map((job) => (
              <tr key={job.id} className="hover:bg-slate-50">
                <td className="px-6 py-4 whitespace-nowrap">
                  <div className="text-sm font-medium text-slate-900">
                    {job.title}
                  </div>
                  {job.description && (
                    <div className="text-xs text-slate-500 truncate max-w-xs">
                      {job.description}
                    </div>
                  )}
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span className="inline-flex items-center rounded-full bg-slate-100 px-2.5 py-0.5 text-xs font-medium text-slate-800">
                    {job.status}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-600">
                  <div className="flex items-center">
                    <Briefcase className="mr-1.5 h-3 text-slate-400" />
                    <span className="font-mono text-xs">
                      {job.service_id.slice(0, 8)}...
                    </span>
                  </div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-600">
                  {job.project_id ? (
                    <span className="inline-flex items-center text-blue-600 bg-blue-50 px-2 py-1 rounded text-xs">
                      Proj: {job.project_id.slice(0, 8)}...
                    </span>
                  ) : job.theme_id ? (
                    <span className="inline-flex items-center text-purple-600 bg-purple-50 px-2 py-1 rounded text-xs">
                      Theme: {job.theme_id?.slice(0, 8)}...
                    </span>
                  ) : (
                    <span className="text-slate-400">-</span>
                  )}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-500">
                  <div className="flex items-center">
                    <Calendar className="mr-1.5 h-3 w-3 text-slate-400" />
                    {new Date(job.created_at).toLocaleDateString()}
                  </div>
                </td>
              </tr>
            ))}
            {jobs?.length === 0 && (
              <tr>
                <td
                  colSpan={5}
                  className="px-6 py-12 text-center text-slate-500"
                >
                  No jobs found. Create your first job!
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>

      {/* Create Dialog */}
      <CreateJobDialog
        isOpen={isDialogOpen}
        onClose={() => setIsDialogOpen(false)}
      />
    </div>
  );
}
