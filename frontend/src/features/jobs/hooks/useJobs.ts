import { api } from "@/lib/api";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { CreateJobPayload, Job } from "../types";

export function useJons() {
  return useQuery({
    queryKey: ["jobs"],
    queryFn: async () => {
      const { data } = await api.get<Job[]>("/jobs");
      return data;
    },
  });
}

export function useCreateJob(onSuccess?: () => void) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (input: CreateJobPayload) => {
      const { data } = await api.post<Job>("/jobs", input);
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["jobs"] });
      if (onSuccess) onSuccess();
    },
  });
}
