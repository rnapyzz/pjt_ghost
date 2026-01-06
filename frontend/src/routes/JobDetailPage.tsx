// src/routes/JobDetailPage.tsx
import { useParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { getJob } from "@/features/jobs/api/getJob";
import { getItems } from "@/features/items/api/itemOperations";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { BudgetGrid } from "@/features/items/components/BudgetGrid";

export const JobDetailPage = () => {
  const { projectId, jobId } = useParams();

  // 1. Job情報の取得
  const {
    data: job,
    isLoading: isLoadingJob,
    isError: isErrorJob,
    error: errorJob,
  } = useQuery({
    queryKey: ["job", jobId],
    queryFn: () => getJob(projectId!, jobId!),
    enabled: !!projectId && !!jobId,
  });

  // 2. Items(予実データ)の取得
  const {
    data: items,
    isLoading: isLoadingItems,
    isError: isErrorItems,
    error: errorItems,
  } = useQuery({
    queryKey: ["items", jobId],
    queryFn: () => getItems(projectId!, jobId!),
    enabled: !!projectId && !!jobId,
  });

  // ローディング表示
  if (isLoadingJob || isLoadingItems) {
    return <div className="p-10 text-center">Loading...</div>;
  }

  // エラー表示
  if (isErrorJob || isErrorItems) {
    return (
      <div className="p-10 text-center text-red-500">
        <h2 className="text-xl font-bold">Error fetching data</h2>
        <p>
          {isErrorJob
            ? errorJob instanceof Error
              ? errorJob.message
              : "Job Error"
            : errorItems instanceof Error
            ? errorItems.message
            : "Items Error"}
        </p>
      </div>
    );
  }

  // Jobが見つからない場合
  if (!job) {
    return <div className="p-10 text-center">Job not found</div>;
  }

  return (
    <div className="space-y-6">
      {/* ヘッダーエリア */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">{job.name}</h1>
          <div className="flex items-center gap-3 mt-2">
            {/* ビジネスモデルをバッジで表示 */}
            <Badge variant="secondary" className="text-sm px-3 uppercase">
              {job.business_model}
            </Badge>
            <span className="text-muted-foreground text-sm">
              Project ID: {projectId}
            </span>
          </div>
        </div>
        <div className="text-right">
          <p className="text-sm text-muted-foreground">
            Created: {new Date(job.created_at).toLocaleDateString()}
          </p>
        </div>
      </div>

      {/* メインエリア：予実管理グリッド */}
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>予実管理 (2026年度)</CardTitle>

          {/* 項目追加ボタン（ロジックはまだ未実装なので見た目だけ） */}
          <Button size="sm">+ Add Item</Button>
        </CardHeader>
        <CardContent>
          {/* 作成したグリッドコンポーネントを表示 */}
          <BudgetGrid items={items || []} />
        </CardContent>
      </Card>
    </div>
  );
};
