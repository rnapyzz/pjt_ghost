import type { JsonObject } from "@/types";

export const ProjectType = {
  Normal: "Normal",
  Agile: "Agile",
  Maintenance: "Maintenance",
  RandD: "RandD",
  Operation: "Operation",
  Stock: "Stock",
} as const;

export type ProjectType = (typeof ProjectType)[keyof typeof ProjectType];

// APIから取得できるデータ構造
export type Project = {
  id: string;
  theme_id?: string;
  name: string;
  description?: string;
  project_type: ProjectType | string;
  attributes: JsonObject;
  is_active: boolean;
  owner_id?: string;
  created_by?: string;
  updated_by?: string;
  created_at: string;
  updated_at: string;
};

// 新規作成時のデータ構造
export type CreateProjectPayload = {
  theme_id?: string;
  name: string;
  description?: string;
  attributes?: JsonObject;
  type: ProjectType;
  target_market?: string;
  value_prop?: string;
  target_client?: string;
  kpis?: string;
  owner_id?: string;
};

// 更新時のデータ構造
export type UpdateProjectPayload = {
  theme_id?: string;
  name?: string;
  description?: string;
  attributes?: JsonObject;
  type?: ProjectType;
  target_market?: string;
  value_porp?: string;
  target_client?: string;
  kpis?: string;
  is_active?: boolean;
  owner_id?: string;
};
