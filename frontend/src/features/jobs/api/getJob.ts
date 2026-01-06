import { api } from "@/lib/api";
import type { Job } from "@/types";

export const getJob = async (
  projectId: string,
  jobId: string
): Promise<Job> => {
  const response = await api.get<Job>(`/projects/${projectId}/jobs/${jobId}`);
  return response.data;
};
