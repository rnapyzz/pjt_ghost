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

export type CreateProjectPayload = {
  theme_id?: string;
  name: string;
  description?: string;
  type: ProjectType;
  attributes?: JsonObject;
  owner_id?: string;
};
