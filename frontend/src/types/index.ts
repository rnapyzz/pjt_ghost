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

export type Account = {
  id: string;
  category: string;
  name: string;
};

export type ItemType = {
  id: string;
  account_id: string;
  name: string;
};

export type Entry = {
  item_id: string;
  date: string;
  amount: number;
};

export type Item = {
  id: string;
  job_id: string;
  item_type_id: string;
  assignee_id: string | null;
  name: string;
  description: string;
  entries: Entry[];
  created_at: string;
  updated_at: string;
};
