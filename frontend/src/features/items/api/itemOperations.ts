import { api } from "@/lib/api";
import type { Account, Item, ItemType } from "@/types";

// マスタデータの取得：勘定科目一覧
export const getAccounts = async (): Promise<Account[]> => {
  const response = await api.get<Account[]>("/accounts");
  return response.data;
};

// マスタデータの取得：項目種別一覧
export const getItemTypes = async (accountId?: string): Promise<ItemType[]> => {
  const params = accountId ? { account_id: accountId } : {};
  const response = await api.get<ItemType[]>("/item_types", { params });
  return response.data;
};

// Jobに紐づくItem一覧を取得
export const getItems = async (
  projectId: string,
  jobId: string
): Promise<Item[]> => {
  const response = await api.get<Item[]>(
    `/projects/${projectId}/jobs/${jobId}/items`
  );
  return response.data;
};
