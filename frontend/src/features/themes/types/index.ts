export type Theme = {
  id: string;
  title: string;
  description?: string;
  is_active: boolean;
  segment_id?: string;

  created_by?: string;
  updated_by?: string;
  created_at: string;
  updated_at: string;
};

export type CreateThemePayload = {
  title: string;
  description?: string;
  segment_id?: string;
};
