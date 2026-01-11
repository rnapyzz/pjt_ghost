import { Link } from "react-router-dom";
import { ChevronDown, ChevronUp, ExternalLink } from "lucide-react";

import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import type { Project } from "@/types";
import { useJobs } from "@/features/jobs/api/getJobs";

interface ProjectCardProps {
  project: Project;
  isExpanded: boolean;
  onToggleExpand: () => void;
}

export const ProjectCard = ({
  project,
  isExpanded,
  onToggleExpand,
}: ProjectCardProps) => {
  const { data: jobs, isLoading } = useJobs({
    projectId: project.id,
    enabled: isExpanded,
  });

  return (
    <Card
      className={`
        transition-all duration-300 ease-in-out border-muted flex flex-col relative
        ${
          isExpanded
            ? "col-span-full border-primary ring-2 ring-primary/20 shadow-xl min-h-100"
            : "hover:shadow-lg hover:border-primary/50 h-55"
        }
      `}
    >
      {/* ヘッダー: pb-0 で下の余白を完全削除
        space-y もなくして、タイトルと日付を密着させる 
      */}
      <CardHeader className="pb-0">
        <div className="flex items-start justify-between">
          <div className="flex flex-col pr-8">
            {" "}
            {/* space-y を削除し flex-col のみに */}
            <CardTitle className="text-lg font-bold flex items-center gap-2 leading-tight mb-1">
              {/* mb-1 で日付との間にわずかな隙間だけ確保 */}
              <span className="line-clamp-1" title={project.name}>
                {project.name}
              </span>
              {isExpanded && (
                <Badge
                  variant="outline"
                  className="text-[10px] px-1 font-normal shrink-0"
                >
                  Editing
                </Badge>
              )}
            </CardTitle>
            <CardDescription className="text-[10px] text-muted-foreground/70">
              Updated: {new Date(project.updated_at).toLocaleDateString()}
            </CardDescription>
          </div>

          <Button
            variant="ghost"
            size="icon"
            className="h-6 w-6 text-muted-foreground"
            asChild
            title="Open Details Page"
          >
            <Link to={`/projects/${project.id}`}>
              <ExternalLink className="h-3 w-3" />
            </Link>
          </Button>
        </div>
      </CardHeader>

      {/* コンテンツ: pt-2 (8px) 程度でヘッダーと分離
        divで囲まず直接 p タグに line-clamp を当てることで ... を確実に表示
      */}
      <CardContent className="flex-1 overflow-hidden py-0 px-6 pt-2">
        {!isExpanded && (
          <p className="text-xs text-muted-foreground whitespace-pre-wrap leading-relaxed line-clamp-5">
            {/* line-clamp-5: 5行を超えたら ... にする。
                h-[220px] の場合、だいたい5〜6行入るので、これで ... が見えるはずです */}
            {project.description || "No description provided."}
          </p>
        )}

        {isExpanded && (
          <div className="animate-in fade-in zoom-in-95 duration-200 h-full">
            {project.description && (
              <p className="text-sm text-muted-foreground mb-6 whitespace-pre-wrap">
                {project.description}
              </p>
            )}

            <div className="flex justify-between items-center mb-4">
              <h3 className="text-lg font-semibold">Jobs / Items</h3>
              <Button size="sm" className="h-8 text-xs">
                + Add Job
              </Button>
            </div>

            <div className="bg-muted/30 rounded-md border p-4">
              {isLoading ? (
                <div className="flex justify-center p-4 text-sm">
                  Loading...
                </div>
              ) : jobs?.length === 0 ? (
                <div className="text-center text-muted-foreground py-8 text-sm">
                  No jobs found. Start by adding one!
                </div>
              ) : (
                <div className="space-y-2">
                  {jobs?.map((job) => (
                    <div
                      key={job.id}
                      className="flex items-center justify-between p-2 bg-background border rounded-md shadow-sm text-sm"
                    >
                      <span className="font-medium">{job.name}</span>
                      <div className="flex gap-4 text-muted-foreground">
                        <span className="px-2 py-0.5 bg-secondary rounded text-[10px]">
                          Contract
                        </span>
                        <span>¥ ---</span>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        )}
      </CardContent>

      <CardFooter className="p-2 pt-0 mt-auto bg-transparent z-10">
        <Button
          variant="ghost"
          className="w-full h-8 text-xs text-muted-foreground hover:bg-secondary/50 hover:text-primary transition-colors"
          onClick={(e) => {
            e.preventDefault();
            onToggleExpand();
          }}
        >
          {isExpanded ? (
            <>
              <ChevronUp className="h-3 w-3 mr-1" /> Close
            </>
          ) : (
            <>
              <ChevronDown className="h-3 w-3 mr-1" /> Show Jobs
            </>
          )}
        </Button>
      </CardFooter>
    </Card>
  );
};
