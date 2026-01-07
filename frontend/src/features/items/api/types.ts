export type CreateItemDTO = {
  item_type_id: string;
  name: string;
  description?: string;
  assignee_id?: string | null;
  entries: { date: string; amount: number }[];
};

export type UpdateItemDTO = {
  name?: string;
  description?: string;
  assignee_id?: string | null;
};

export type EntryDTO = {
  date: string;
  amount: number;
};

export type UpdateEntriesDTO = {
  entries: EntryDTO[];
};
