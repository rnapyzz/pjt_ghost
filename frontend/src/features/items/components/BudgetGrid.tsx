import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
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

type Props = {
  items: Item[];
};

export const BudgetGrid = ({ items }: Props) => {
  const { projectId, jobId } = useParams();
  const months = generateMonthColumns("2026-04-01");
  const { mutate, isPending } = useUpdateEntries(projectId!, jobId!);

  // ▼ State管理
  const [isEditing, setIsEditing] = useState(false); // 編集モードフラグ
  const [draftItems, setDraftItems] = useState<Item[]>([]); // 編集中のデータ

  // 初期ロード時や保存完了時に、ドラフトをサーバーデータと同期
  useEffect(() => {
    setDraftItems(items);
  }, [items]);

  // ▼ ハンドラ: 編集モード開始
  const startEditing = () => {
    setDraftItems(JSON.parse(JSON.stringify(items))); // 深いコピーで切り離す
    setIsEditing(true);
  };

  // ▼ ハンドラ: キャンセル
  const cancelEditing = () => {
    setDraftItems(items); // 元に戻す
    setIsEditing(false);
  };

  // ▼ ハンドラ: 入力変更 (ローカルStateのみ更新)
  const handleInputChange = (
    itemIndex: number,
    dateKey: string,
    valStr: string
  ) => {
    const newVal = parseInt(valStr, 10);
    if (isNaN(newVal) && valStr !== "") return; // 数字以外は無視 (空文字は許容)

    setDraftItems((prev) => {
      const next = [...prev];
      const targetItem = { ...next[itemIndex] };
      const entries = [...targetItem.entries];

      const entryIndex = entries.findIndex((e) => e.date === dateKey);
      if (entryIndex >= 0) {
        // 更新
        entries[entryIndex] = {
          ...entries[entryIndex],
          amount: isNaN(newVal) ? 0 : newVal,
        };
      } else {
        // 新規
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

  // ▼ ハンドラ: 一括保存
  const handleSave = () => {
    // 変更があったItemだけを抽出してAPIを叩くループ処理
    // ※ 理想は「一括更新API」を作るべきですが、既存APIを並列で叩く簡易実装にします
    const promises = draftItems.map((draftItem) => {
      // 本来は変更検知(diff)をして、変更があったものだけ送るのが良い
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

    // 全てのリクエストが終わったらモードを戻す
    // (useMutationのonSuccessでinvalidateしているので、画面は自動で最新になる)
    Promise.all(promises).then(() => {
      setIsEditing(false);
    });
  };

  return (
    <div className="space-y-4">
      {/* ツールバーエリア */}
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
              {isPending ? "保存中..." : "保存して確定"}
            </Button>
          </>
        ) : (
          <Button onClick={startEditing} variant="secondary">
            編集モードにする
          </Button>
        )}
      </div>

      <div className="border rounded-md overflow-x-auto">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-[200px] min-w-[200px] bg-muted/50 sticky left-0 z-10">
                項目名
              </TableHead>
              {months.map((month) => (
                <TableHead key={month.key} className="text-right min-w-[100px]">
                  {month.label}
                </TableHead>
              ))}
            </TableRow>
          </TableHeader>
          <TableBody>
            {/* 表示には draftItems を使う */}
            {draftItems.map((item, idx) => (
              <TableRow key={item.id}>
                <TableCell className="font-medium bg-background sticky left-0 z-10 border-r">
                  {item.name}
                </TableCell>

                {months.map((month) => {
                  const entry = item.entries.find((e) => e.date === month.key);
                  const amount = entry?.amount ?? 0;

                  return (
                    <TableCell key={month.key} className="p-1 text-right">
                      {isEditing ? (
                        <Input
                          className="text-right h-8"
                          // 0の場合は空文字にして入力しやすくする等の工夫も可
                          value={amount}
                          onChange={(e) =>
                            handleInputChange(idx, month.key, e.target.value)
                          }
                        />
                      ) : (
                        // 通常時はテキスト表示
                        <span className="block px-2 py-1">
                          {formatCurrency(amount)}
                        </span>
                      )}
                    </TableCell>
                  );
                })}
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    </div>
  );
};
