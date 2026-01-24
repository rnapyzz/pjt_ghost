import { EditJobDialog } from "@/features/jobs/components/EditJobDialog";
import { useDeleteJob, useJob } from "@/features/jobs/hooks/useJobs";
import { useProjects } from "@/features/projects/hooks/useProjects";
import { useServices } from "@/features/services/hooks/useServices";
import { useThemes } from "@/features/themes/hooks/useThemes";
import {
  ArrowLeft,
  Box,
  Calendar,
  Clock,
  FlagTriangleRight,
  FolderKanban,
  Loader2,
  MoreHorizontal,
  Pencil,
  Trash,
} from "lucide-react";
import { useState } from "react";
import { Link, useParams } from "react-router-dom";

export function JobDetailPage() {
  const { jobId } = useParams<{ jobId: string }>();
  const { data: job, isLoading } = useJob(jobId || "");

  const { mutate: deleteJob, isPending: isDeleting } = useDeleteJob();
  const [isEditOpen, setIsEditOpen] = useState(false);

  const { data: services } = useServices();
  const { data: projects } = useProjects();
  const { data: themes } = useThemes();

  const serviceName = services?.find((s) => s.id === job?.service_id)?.name;
  const projectName = projects?.find((p) => p.id === job?.project_id)?.name;
  const themeName = themes?.find((t) => t.id === job?.theme_id)?.title;

  const handleDelete = () => {
    if (!job) return;
    if (
      window.confirm(
        "Are you sure you want to delete this job? This action cannotbe undone.",
      )
    ) {
      deleteJob(job.id);
    }
  };

  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <Loader2 className="animate-spin text-slate-400" />
      </div>
    );
  }

  if (!job) {
    return <div>Job not found.</div>;
  }

  return (
    <div className="flex flex-col h-full max-w-5xl mx-auto">
      {/* header */}
      <div className="mb-6">
        <Link
          to="/"
          className="inline-flex items-center text-sm text-slate-500 hover:text-slate-800 transition-colors mb-4"
        >
          <ArrowLeft className="mr-1 h-4 w-4" />
          Back to Matrix
        </Link>

        <div className="flex items-start justify-between">
          {/* title */}
          <div className="flex items-center gap-3 mb-2">
            <h1 className="text-2xl font-semibold text-slate-900">
              {job.title}
            </h1>
            <span
              className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${job.status === "Completed" ? "bg-green-100 text-green-800" : "bg-blue-100 text-blue-800"}`}
            >
              {job.status}
            </span>
          </div>

          {/* buttons */}
          <div className="flex gap-2">
            <button
              onClick={() => setIsEditOpen(true)}
              className="flex items-center rounded-md border border-slate-300 bg-white px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50"
            >
              <Pencil className="mr-2 h-4 w-4" />
              Edit
            </button>
            <button
              onClick={handleDelete}
              disabled={isDeleting}
              className="flex items-center rounded-md border border-slate-300 bg-white px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50"
            >
              <Trash className="mr-2 h-4 w-4" />
              Delete
            </button>
            <button className="flex items-center rounded-md border border-slate-300 bg-white px-3 py-2 text-sm font-medium text-slate-700 hover:bg-slate-50">
              <MoreHorizontal className="h-4 w-4" />
            </button>
          </div>
        </div>
      </div>
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-3">
        {/* left */}
        <div className="lg:col-span-2 space-y-2">
          <section className="bg-white rounded-lg border border-slate-200 p-6 shadow-sm">
            <h2 className="text-sm font-semibold text-slate-900 mb-4 border-b pb-2">
              Description
            </h2>
            <div className="prose prose-slate max-w-none text-slate-600 text-sm whitespace-pre-wrap">
              {job?.description || (
                <span className="italic text-sm text-slate-400">
                  No description provided.
                </span>
              )}
            </div>
          </section>

          <section className="bg-white rounded-lg border border-slate-200 border-dashed p-6 shadow-sm min-h-50 flex items-center justify-center">
            <div className="text-center text-slate-400">
              <p className="text-sm">activity log & subtask coming soon...</p>
            </div>
          </section>
        </div>

        {/* right */}
        <div>
          <div className="space-y-4">
            <div className="bg-slate-50 rounded-lg border border-slate-200 p-5">
              <h3 className="text-xs font-semibold uppercase tracking-wider text-slate-500 mb-3">
                Context
              </h3>
              <div className="space-y-4">
                {/* service */}
                <div className="flex items-start">
                  <Box className="mr-3 h-5 w-5 text-slate-400 mt-0.5" />
                  <div>
                    <p className="text-xs text-slate-500">Service</p>
                    <p className="text-sm font-medium text-slate-900">
                      {serviceName}
                    </p>
                  </div>
                </div>

                {/* project */}
                {projectName && (
                  <div className="flex items-start">
                    <FolderKanban className="mr-3 h-5 w-5 text-slate-400 mt-0.5" />
                    <div>
                      <p className="text-xs text-slate-500">Project</p>
                      <p className="text-sm font-medium text-slate-900">
                        {projectName ? projectName : "No Project"}
                      </p>
                    </div>
                  </div>
                )}

                {/* theme */}
                {themeName && (
                  <div className="flex items-start">
                    <FlagTriangleRight className="mr-3 h-5 w-5 text-slate-400 mt-0.5" />
                    <div>
                      <p className="text-xs text-slate-500">Theme</p>
                      <p className="text-sm font-medium text-slate-900">
                        {themeName ? themeName : "No Theme"}
                      </p>
                    </div>
                  </div>
                )}
              </div>
            </div>

            {/* timestamp card */}
            <div className="bg-white rounded-lg border border-slate-200 p-5 space-y-2">
              <div className="flex items-center text-xs text-slate-400">
                <Calendar className="mr-3 h-3 w-3" />
                <span>
                  Created:{" "}
                  {job ? new Date(job.created_at).toLocaleDateString() : "-"}
                </span>
              </div>
              <div className="flex items-center text-xs text-slate-400">
                <Clock className="mr-3 h-3 w-3" />
                <span>
                  Updated:{" "}
                  {job ? new Date(job.updated_at).toLocaleDateString() : "-"}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {isEditOpen && job && (
        <EditJobDialog
          job={job}
          isOpen={true}
          onClose={() => setIsEditOpen(false)}
        />
      )}
    </div>
  );
}
