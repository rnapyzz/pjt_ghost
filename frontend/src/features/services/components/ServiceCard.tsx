import { Box, Calendar, Hash } from "lucide-react";
import type { Service } from "../types";

type Props = {
  service: Service;
  onClick: (slug: string) => void;
};

export function ServiceCard({ service, onClick }: Props) {
  return (
    <div
      onClick={() => onClick(service.slug)}
      className="group flex flex-col justify-between rounded-xl border border-slate-200 bg-white p-6 shadow-sm transition-all hover:shadow-md hover:border-blue-200 cursor-pointer"
    >
      <div>
        <div className="mb-4 flex h-10 w-10 items-center justify-center rounded-lg bg-blue-50 text-blue-600 group-hover:bg-blue-100 group-hover:text-blue-700 transition-colors">
          <Box className="h-6 w-6" />
        </div>
        <h3 className="text-lg font-bold text-slate-900">{service.name}</h3>

        <div className="mt-6 border-t border-slate-100 pt-4 flex items-center text-xs text-slate-400 gap-2">
          <Hash className="h-3 w-3" />
          <span>{service.slug}</span>
        </div>
      </div>

      <div className="mt-6 border-t border-slate-100 pt-4 flex items-center text-xs text-slate-400 gap-2">
        <Calendar className="h-3 w-3" />
        <span>{new Date(service.created_at).toLocaleDateString()}</span>
      </div>
    </div>
  );
}
