import { CreateProjectDialog } from "@/features/projects/components/CreateProjectDialog";
import { useState } from "react";

export function ProjectsPage() {
  const [isDialogOpen, setIsDialogOpen] = useState(true);

  return (
    <CreateProjectDialog
      isOpen={isDialogOpen}
      onClose={() => setIsDialogOpen(false)}
    />
  );
}
