import { api } from "@/lib/api";
import type { Job } from "@/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

export type CreateJobDTO = {
  name: string;
  business_model: "contract" | "ses" | "saas" | "media" | "internal" | "rnd";
};

export const createJob = async ({
  projectId,
  data,
}: {
  projectId: string;
  data: CreateJobDTO;
}): Promise<Job> => {
  const response = await api.post<Job>(`/projects/${projectId}/jobs`, data);
  return response.data;
};

export const useCreateJob = (projectId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: CreateJobDTO) => createJob({ projectId, data }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["jobs", projectId] });
    },
  });
};
