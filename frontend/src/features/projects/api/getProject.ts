import { api } from "@/lib/api";
import type { Project } from "@/types";

// projectIdからprojectの詳細を取得する関数
export const getProject = async (id: string): Promise<Project> => {
  const response = await api.get(`/projects/${id}`);
  return response.data;
};
