import { api } from "@/lib/api";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { CreateJobPayload, Job, UpdateJobPayload } from "../types";
import { useNavigate } from "react-router-dom";

export function useJobs() {
  return useQuery({
    queryKey: ["jobs"],
    queryFn: async () => {
      const { data } = await api.get<Job[]>("/jobs");
      return data;
    },
  });
}

export function useJob(jobId: string) {
  return useQuery({
    queryKey: ["jobs", jobId],
    queryFn: async () => {
      const { data } = await api.get<Job>(`/jobs/${jobId}`);
      return data;
    },
    enabled: !!jobId,
  });
}

export function useCreateJob(onSuccess?: () => void) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (payload: CreateJobPayload) => {
      const { data } = await api.post<Job>("/jobs", payload);
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["jobs"] });
      if (onSuccess) onSuccess();
    },
  });
}

export function useUpdateJob(onSuccess?: () => void) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (payload: UpdateJobPayload) => {
      const { id, ...body } = payload;
      const { data } = await api.put<Job>(`/jobs/${id}`, body);
      return data;
    },
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: ["jobs"] });
      queryClient.invalidateQueries({ queryKey: ["jobs", data.id] });
      if (onSuccess) onSuccess();
    },
    onError: (error) => {
      console.error("Update Error: ", error);
      alert("Failed to update job.");
    },
  });
}

export function useDeleteJob() {
  const queryClient = useQueryClient();
  const navigate = useNavigate();

  return useMutation({
    mutationFn: async (jobId: string) => {
      await api.delete(`/jobs/${jobId}`);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["jobs"] });
      navigate("/");
    },
    onError: (error) => {
      console.error("Delete Error: ", error);
      alert("Failed to delete job.");
    },
  });
}
