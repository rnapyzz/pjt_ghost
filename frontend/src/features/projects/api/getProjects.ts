import { api } from "@/lib/api";
import type { Project } from "@/types";
import { useQuery } from "@tanstack/react-query";

const getProjects = async (): Promise<Project[]> => {
  const response = await api.get("/projects");
  return response.data;
};

export const useProjects = () => {
  return useQuery({
    queryKey: ["projects"],
    queryFn: getProjects,
  });
};
