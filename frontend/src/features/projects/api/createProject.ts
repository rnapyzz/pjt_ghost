import { api } from "@/lib/api";
import type { Project } from "@/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

export type CreateProjectDTO = {
  name: string;
  description: string;
};

const createProject = async (data: CreateProjectDTO): Promise<Project> => {
  const response = await api.post("/projects", data);
  return response.data;
};

export const useCreateProject = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: createProject,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["projects"] });
    },
  });
};
