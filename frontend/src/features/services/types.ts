export type Service = {
  id: string;
  slug: string;
  name: string;
  owner_id?: string;

  created_by?: string;
  updated_by?: string;
  created_at: string;
  updated_at: string;
};

export type CreateServiceInput = {
  name: string;
  slug?: string;
};
