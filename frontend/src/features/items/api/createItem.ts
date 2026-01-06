import { api } from "@/lib/api";
import type { Item } from "@/types";
import { useMutation, useQueryClient } from "@tanstack/react-query";

export type CreateItemDTO = {
  item_type_id: string;
  name: string;
  description?: string;
  assignee_id?: string | null;
  entries: { date: string; amount: number }[];
};

export const createItem = async ({
  projectId,
  jobId,
  data,
}: {
  projectId: string;
  jobId: string;
  data: CreateItemDTO;
}): Promise<Item> => {
  const response = await api.post<Item>(
    `/projects/${projectId}/jobs/${jobId}/items`,
    data
  );
  return response.data;
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
