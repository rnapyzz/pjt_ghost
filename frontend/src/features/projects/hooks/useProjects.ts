import type { CreateProjectPayload, UpdateProjectPayload } from "../types";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { createProject, getProject, getProjects, updateProject } from "../api";

export function useProjects() {
  return useQuery({
    queryKey: ["projects"],
    queryFn: getProjects,
  });
}

export function useProject(id: string) {
  return useQuery({
    queryKey: ["project", id],
    queryFn: () => getProject(id),
    enabled: !!id,
  });
}

export function useCreateProject(onSuccess?: () => void) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: CreateProjectPayload) => createProject(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["projects"] });
      if (onSuccess) onSuccess();
    },
    onError: (error) => {
      console.error("Crate Project Error:", error);
    },
  });
}

export function useUpdateProject(onSuccess?: () => void) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateProjectPayload }) =>
      updateProject({ id, data }),
    onSuccess: (data) => {
      queryClient.setQueryData(["projects", data.id], data);
      queryClient.invalidateQueries({ queryKey: ["projects"] });
      if (onSuccess) onSuccess();
    },
  });
}
