import { api } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";

type EntryDTO = {
  date: string;
  amount: number;
};

type UpdateEntriesPayload = {
  projectId: string;
  jobId: string;
  itemId: string;
  entries: EntryDTO[];
};

export const updateEntries = async ({
  projectId,
  jobId,
  itemId,
  entries,
}: UpdateEntriesPayload): Promise<void> => {
  await api.put(
    `/projects/${projectId}/jobs/${jobId}/items/${itemId}/entries`,
    { entries }
  );
};

export const useUpdateEntries = (_projectId: string, jobId: string) => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: updateEntries,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["items", jobId] });
    },
  });
};
