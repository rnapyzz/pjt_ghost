import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";
import type { Job } from "@/types";

// 1. 既存のFetcher関数（そのまま活かします）
// ※戻り値の型安全のため <Job[]> を明示しておくとより良いです
export const getJobs = async (projectId: string): Promise<Job[]> => {
  const response = await api.get<Job[]>(`/projects/${projectId}/jobs`);
  return response.data;
};

// 2. 今回追加するHook
// ProjectCardで「開いたときだけ取得」するために enabled オプションを受け取れるようにします
type UseJobsOptions = {
  projectId: string;
  enabled?: boolean;
};

export const useJobs = ({ projectId, enabled = true }: UseJobsOptions) => {
  return useQuery({
    queryKey: ["jobs", projectId],
    queryFn: () => getJobs(projectId), // 上で定義した関数を再利用
    enabled, // これが false の時はリクエストが飛びません
  });
};
