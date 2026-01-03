import { api } from "@/lib/api";
import type { Project } from "@/types";
import { useQuery } from "@tanstack/react-query";

// useQueryで使用するプロジェクトデータの取得用関数
const getProjects = async (): Promise<Project[]> => {
  const response = await api.get("/projects");
  return response.data;
};

// コンポーネントで使用するuseQueryを使ったプロジェクトデータを取得するカスタムフック
// useQueryはdata, isLoading, errorなどの状態を管理してくれる
export const useProjects = () => {
  return useQuery({
    // queryKey: キャッシュを管理するためのユニークキー（配列で使う）
    queryKey: ["projects"],
    // queryFn: データを取得するための非同期関数
    queryFn: getProjects,
  });
};
