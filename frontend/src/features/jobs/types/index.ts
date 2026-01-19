export type Job = {
  id: string;
  service_id: string;
  project_id?: string;
  theme_id?: string;
  title: string;
  description?: string;
  status: string;
  owner_id?: string;
  created_by?: string;
  updated_by?: string;
  created_at: string;
  updated_at: string;
};

export type CreateJobPayload = {
  service_id: string;
  project_id?: string;
  theme_id?: string;
  title: string;
  description?: string;
  status?: string;
  owner_id?: string;
};
