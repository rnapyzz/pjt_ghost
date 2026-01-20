import { api } from "@/lib/api";
import type { CreateProjectPayload, Project } from "../types";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export function useProjects() {
  return useQuery({
    queryKey: ["projects"],
    queryFn: async () => {
      const { data } = await api.get<Project[]>("/projects");
      return data;
    },
  });
}

export function useCreateProject(onSuccess?: () => void) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (input: CreateProjectPayload) => {
      const { data } = await api.post<Project>("/projects", input);
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["projects"] });
      if (onSuccess) onSuccess();
    },
  });
}
