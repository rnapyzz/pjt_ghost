import { api } from "@/lib/api";
import type { Item, Account, ItemType } from "@/types";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { CreateItemDTO, UpdateItemDTO, UpdateEntriesDTO } from "./types";

// ==========================================
// Read Operations (Queries)
// ==========================================

// Jobに紐づくItems一覧取得
export const getItems = async (
  projectId: string,
  jobId: string
): Promise<Item[]> => {
  const res = await api.get<Item[]>(
    `/projects/${projectId}/jobs/${jobId}/items`
  );
  return res.data;
};

export const useItems = (projectId: string, jobId: string) => {
  return useQuery({
    queryKey: ["items", jobId],
    queryFn: () => getItems(projectId, jobId),
    enabled: !!projectId && !!jobId,
  });
};

// マスタ取得: Accounts
export const getAccounts = async (): Promise<Account[]> => {
  const res = await api.get<Account[]>("/accounts");
  return res.data;
};

export const useAccounts = () => {
  return useQuery({
    queryKey: ["accounts"],
    queryFn: getAccounts,
    staleTime: Infinity,
  });
};

// マスタ取得: ItemTypes
export const getItemTypes = async (accountId?: string): Promise<ItemType[]> => {
  const params = accountId ? { account_id: accountId } : {};
  const res = await api.get<ItemType[]>("/item-types", { params });
  return res.data;
};

export const useItemTypes = (accountId?: string) => {
  return useQuery({
    queryKey: ["itemTypes", accountId],
    queryFn: () => getItemTypes(accountId),
  });
};

// ==========================================
// Write Operations (Mutations)
// ==========================================

// Create Item
export const createItem = async ({
  projectId,
  jobId,
  data,
}: {
  projectId: string;
  jobId: string;
  data: CreateItemDTO;
}): Promise<Item> => {
  const res = await api.post<Item>(
    `/projects/${projectId}/jobs/${jobId}/items`,
    data
  );
  return res.data;
};

export const useCreateItem = (projectId: string, jobId: string) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateItemDTO) => createItem({ projectId, jobId, data }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["items", jobId] });
    },
  });
};

// Update Item (Name, Description etc.)
export const updateItem = async ({
  projectId,
  jobId,
  itemId,
  data,
}: {
  projectId: string;
  jobId: string;
  itemId: string;
  data: UpdateItemDTO;
}) => {
  const res = await api.put<Item>(
    `/projects/${projectId}/jobs/${jobId}/items/${itemId}`,
    data
  );
  return res.data;
};

export const useUpdateItem = (projectId: string, jobId: string) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (args: { itemId: string; data: UpdateItemDTO }) =>
      updateItem({ projectId, jobId, itemId: args.itemId, data: args.data }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["items", jobId] });
    },
  });
};

// Delete Item
export const deleteItem = async ({
  projectId,
  jobId,
  itemId,
}: {
  projectId: string;
  jobId: string;
  itemId: string;
}) => {
  await api.delete(`/projects/${projectId}/jobs/${jobId}/items/${itemId}`);
};

export const useDeleteItem = (projectId: string, jobId: string) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (itemId: string) => deleteItem({ projectId, jobId, itemId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["items", jobId] });
    },
  });
};

// Update Entries (Budget Numbers)
export const updateEntries = async ({
  projectId,
  jobId,
  itemId,
  data,
}: {
  projectId: string;
  jobId: string;
  itemId: string;
  data: UpdateEntriesDTO;
}): Promise<void> => {
  await api.put(
    `/projects/${projectId}/jobs/${jobId}/items/${itemId}/entries`,
    data
  );
};

export const useUpdateEntries = (projectId: string, jobId: string) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (args: { itemId: string; data: UpdateEntriesDTO }) =>
      updateEntries({ projectId, jobId, itemId: args.itemId, data: args.data }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["items", jobId] });
    },
  });
};
