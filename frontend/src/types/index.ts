export type Project = {
  id: string;
  name: string;
  description: string;
  owner_id: string;
  created_at: string;
  updated_at: string;
};

export type Job = {
  id: string;
  project_id: string;
  name: string;
  business_model: string;
  created_at: string;
  updated_at: string;
};
