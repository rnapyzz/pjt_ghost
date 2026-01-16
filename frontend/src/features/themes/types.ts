export type Theme = {
  id: string;
  title: string;
  description?: string;
  is_active: boolean;

  created_by?: string;
  updated_by?: string;
  created_at: string;
  updated_at: string;
};

export type CreateThemeInput = {
  title: string;
  description?: string;
};
