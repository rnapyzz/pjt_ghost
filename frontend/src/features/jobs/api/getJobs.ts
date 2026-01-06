import { api } from "@/lib/api";
import type { Job } from "@/types";

export const getJobs = async (projectId: string): Promise<Job[]> => {
  const response = await api.get(`/projects/${projectId}/jobs`);
  return response.data;
};
