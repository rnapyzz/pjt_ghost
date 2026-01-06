import { useState, useEffect, useMemo } from "react";
import { useParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { type Item } from "@/types";
import { generateMonthColumns, formatCurrency } from "@/lib/dateUtils";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useUpdateEntries } from "../api/updateEntries";
import { getAccounts, getItemTypes } from "../api/itemOperations";

type Props = {
  items: Item[];
};

// 表示順序と日本語ラベルの定義
const CATEGORY_ORDER = [
  "sales",
  "cost_of_sales",
  "sga",
  "non_operating",
] as const;
const CATEGORY_LABELS: Record<string, string> = {
  sales: "売上高",
  cost_of_sales: "売上原価",
  sga: "販管費",
  non_operating: "営業外",
};

export const BudgetGrid = ({ items }: Props) => {
  const { projectId, jobId } = useParams();
  const months = generateMonthColumns("2026-04-01");
  const { mutate, isPending } = useUpdateEntries(projectId!, jobId!);

  // ▼ マスタデータの取得（結合用）
  const { data: accounts } = useQuery({
    queryKey: ["accounts"],
    queryFn: getAccounts,
  });
  const { data: itemTypes } = useQuery({
    queryKey: ["itemTypes"],
    queryFn: () => getItemTypes(),
  });

  // State
  const [isEditing, setIsEditing] = useState(false);
  const [draftItems, setDraftItems] = useState<Item[]>([]);

  useEffect(() => {
    setDraftItems(items);
  }, [items]);

  // ▼▼▼ 集計ロジック (ここが心臓部) ▼▼▼
  const { groupedItems, monthlyTotals, profit } = useMemo(() => {
    if (!accounts || !itemTypes)
      return { groupedItems: {}, monthlyTotals: {}, profit: {} };

    // 1. ItemにAccount情報を紐付ける
    const enrichedItems = draftItems.map((item) => {
      const type = itemTypes.find((t) => t.id === item.item_type_id);
      const account = accounts.find((a) => a.id === type?.account_id);
      return { ...item, account, itemType: type };
    });

    // 2. Accountカテゴリごとにグルーピング
    const groups: Record<string, typeof enrichedItems> = {};
    CATEGORY_ORDER.forEach((cat) => (groups[cat] = []));

    enrichedItems.forEach((item) => {
      const cat = item.account?.category || "non_operating";
      if (!groups[cat]) groups[cat] = [];
      groups[cat].push(item);
    });

    // 3. カテゴリごとの月次合計を計算
    const totals: Record<string, Record<string, number>> = {};

    CATEGORY_ORDER.forEach((cat) => {
      totals[cat] = {};
      months.forEach((m) => {
        const sum = groups[cat].reduce((acc, item) => {
          const entry = item.entries.find((e) => e.date === m.key);
          return acc + (entry?.amount || 0);
        }, 0);
        totals[cat][m.key] = sum;
      });
    });

    // 4. 営業利益の計算 (売上 - 原価 - 販管費)
    const operatingProfit: Record<string, number> = {};
    months.forEach((m) => {
      const sales = totals["sales"][m.key] || 0;
      const cost = totals["cost_of_sales"][m.key] || 0;
      const sga = totals["sga"][m.key] || 0;
      operatingProfit[m.key] = sales - cost - sga;
    });

    return {
      groupedItems: groups,
      monthlyTotals: totals,
      profit: operatingProfit,
    };
  }, [draftItems, accounts, itemTypes, months]);

  // --- ハンドラ類 (前回と同じ) ---
  const startEditing = () => {
    setDraftItems(JSON.parse(JSON.stringify(items)));
    setIsEditing(true);
  };
  const cancelEditing = () => {
    setDraftItems(items);
    setIsEditing(false);
  };
  const handleInputChange = (
    itemIndex: number,
    dateKey: string,
    valStr: string
  ) => {
    // ※ このitemIndexはdraftItems全体の中でのインデックス
    const newVal = parseInt(valStr, 10);
    if (isNaN(newVal) && valStr !== "") return;

    setDraftItems((prev) => {
      const next = [...prev];
      const targetItem = { ...next[itemIndex] };
      const entries = [...targetItem.entries];
      const entryIndex = entries.findIndex((e) => e.date === dateKey);

      if (entryIndex >= 0) {
        entries[entryIndex] = {
          ...entries[entryIndex],
          amount: isNaN(newVal) ? 0 : newVal,
        };
      } else {
        entries.push({
          item_id: targetItem.id,
          date: dateKey,
          amount: isNaN(newVal) ? 0 : newVal,
        });
      }
      targetItem.entries = entries;
      next[itemIndex] = targetItem;
      return next;
    });
  };

  const handleSave = () => {
    const promises = draftItems.map((draftItem) => {
      return mutate({
        projectId: projectId!,
        jobId: jobId!,
        itemId: draftItem.id,
        entries: draftItem.entries.map((e) => ({
          date: e.date,
          amount: e.amount,
        })),
      });
    });
    Promise.all(promises).then(() => setIsEditing(false));
  };

  if (!accounts || !itemTypes) return <div>Loading masters...</div>;

  return (
    <div className="space-y-4">
      {/* ツールバー */}
      <div className="flex justify-end gap-2">
        {isEditing ? (
          <>
            <Button
              variant="outline"
              onClick={cancelEditing}
              disabled={isPending}
            >
              キャンセル
            </Button>
            <Button onClick={handleSave} disabled={isPending}>
              保存して確定
            </Button>
          </>
        ) : (
          <Button onClick={startEditing} variant="secondary">
            編集モードにする
          </Button>
        )}
      </div>

      <div className="border rounded-md overflow-x-auto">
        <Table className="min-w-300">
          <TableHeader>
            <TableRow>
              <TableHead className="w-62.5 min-w-62.5 sticky left-0 z-20 bg-muted/90">
                科目 / 項目
              </TableHead>
              {months.map((month) => (
                <TableHead key={month.key} className="text-right min-w-25">
                  {month.label}
                </TableHead>
              ))}
            </TableRow>
          </TableHeader>

          <TableBody>
            {/* カテゴリごとにループしてレンダリング */}
            {CATEGORY_ORDER.map((category) => {
              const categoryItems = groupedItems[category];
              if (!categoryItems || categoryItems.length === 0) return null;

              return (
                <>
                  {/* --- カテゴリヘッダー (例: 売上高) --- */}
                  <TableRow className="bg-muted/30 hover:bg-muted/30">
                    <TableCell
                      colSpan={months.length + 1}
                      className="font-bold text-base py-3 sticky left-0 z-10 bg-muted/30"
                    >
                      {CATEGORY_LABELS[category]}
                    </TableCell>
                  </TableRow>

                  {/* --- 各項目の行 --- */}
                  {categoryItems.map((item) => {
                    // グローバルなインデックスを探す（編集ハンドラ用）
                    const globalIndex = draftItems.findIndex(
                      (i) => i.id === item.id
                    );

                    return (
                      <TableRow key={item.id}>
                        <TableCell className="sticky left-0 z-10 bg-background border-r">
                          <div className="flex flex-col">
                            <span className="font-medium">{item.name}</span>
                            <span className="text-xs text-muted-foreground">
                              {item.itemType?.name}
                              {/* {item.description && ` - ${item.description}`} */}
                            </span>
                          </div>
                        </TableCell>
                        {months.map((month) => {
                          const entry = item.entries.find(
                            (e) => e.date === month.key
                          );
                          const amount = entry?.amount ?? 0;
                          return (
                            <TableCell
                              key={month.key}
                              className="text-right p-1"
                            >
                              {isEditing ? (
                                <Input
                                  className="text-right h-8"
                                  value={amount}
                                  onChange={(e) =>
                                    handleInputChange(
                                      globalIndex,
                                      month.key,
                                      e.target.value
                                    )
                                  }
                                />
                              ) : (
                                <span className="block px-2">
                                  {formatCurrency(amount)}
                                </span>
                              )}
                            </TableCell>
                          );
                        })}
                      </TableRow>
                    );
                  })}

                  {/* --- カテゴリ小計 --- */}
                  <TableRow className="bg-slate-50 border-t-2 border-slate-200 font-semibold">
                    <TableCell className="text-right sticky left-0 z-10 bg-slate-50 border-r">
                      {CATEGORY_LABELS[category]} 合計
                    </TableCell>
                    {months.map((month) => (
                      <TableCell key={month.key} className="text-right">
                        {formatCurrency(monthlyTotals[category][month.key])}
                      </TableCell>
                    ))}
                  </TableRow>
                </>
              );
            })}

            {/* === 最終行: 営業利益 === */}
            <TableRow className="bg-slate-100 border-t-4 border-slate-300 font-bold text-base">
              <TableCell className="text-right sticky left-0 z-10 bg-slate-100 border-r">
                営業利益
              </TableCell>
              {months.map((month) => {
                const val = profit[month.key];
                return (
                  <TableCell
                    key={month.key}
                    className={`text-right ${
                      val < 0 ? "text-red-600" : "text-blue-700"
                    }`}
                  >
                    {formatCurrency(val)}
                  </TableCell>
                );
              })}
            </TableRow>
          </TableBody>
        </Table>
      </div>
    </div>
  );
};
