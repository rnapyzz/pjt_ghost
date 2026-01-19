export type SegmentUiConfig = {
  icon?: string;
  color_theme?: string;
};

export type Segment = {
  id: string;
  slug: string;
  name: string;
  description?: string;
  ui_config: SegmentUiConfig;
  created_by?: string;
  updated_by?: string;
  created_at: string;
  updated_at: string;
};

export type CreateSegmentPayload = {
  slug: string;
  name: string;
  description?: string;
  ui_config?: SegmentUiConfig;
};
