import { api } from "@/lib/api";
import type {
  CreateProjectPayload,
  Project,
  UpdateProjectPayload,
} from "../types";

// 一覧取得
export const getProjects = async (): Promise<Project[]> => {
  const res = await api.get<Project[]>("/projects");
  return res.data;
};

// 1件取得
export const getProject = async (id: string): Promise<Project> => {
  const res = await api.get<Project>(`/projects/${id}`);
  return res.data;
};

// 新規作成
export const createProject = async (
  data: CreateProjectPayload,
): Promise<Project> => {
  const res = await api.post<Project>("/projects", data);
  return res.data;
};

// 更新
export const updateProject = async ({
  id,
  data,
}: {
  id: string;
  data: UpdateProjectPayload;
}): Promise<Project> => {
  const res = await api.patch<Project>(`/projects/${id}`, data);
  return res.data;
};
