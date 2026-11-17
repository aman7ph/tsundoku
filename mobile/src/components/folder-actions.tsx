import { useState } from "react";

import { ActionRow } from "@/components/action-row";
import { BottomSheet } from "@/components/bottom-sheet";
import { DeleteConfirm } from "@/components/delete-confirm";
import { NameForm } from "@/components/name-sheet";
import { SheetHeader } from "@/components/sheet-header";

export type FolderRef = { id: string; name: string };

type FolderActionsProps = {
  visible: boolean;
  folder: FolderRef | null;
  kind: "collection" | "category";
  onRename: (id: string, name: string) => Promise<unknown>;
  onDelete: (id: string) => Promise<unknown>;
  onClose: () => void;
  onChanged: () => void;
};

export function FolderActions({
  visible,
  folder,
  onClose,
  ...rest
}: FolderActionsProps) {
  return (
    <BottomSheet visible={visible} onClose={onClose}>
      {folder ? <Actions folder={folder} onClose={onClose} {...rest} /> : null}
    </BottomSheet>
  );
}

type ActionsProps = Omit<FolderActionsProps, "visible" | "folder"> & {
  folder: FolderRef;
};

function Actions({
  folder,
  kind,
  onRename,
  onDelete,
  onClose,
  onChanged,
}: ActionsProps) {
  const [view, setView] = useState<"menu" | "rename" | "delete">("menu");

  if (view === "rename") {
    return (
      <NameForm
        title={`Rename ${kind}`}
        placeholder="Name"
        submitLabel="Save"
        initialValue={folder.name}
        onSubmit={(name) => onRename(folder.id, name)}
        onClose={onClose}
        onDone={onChanged}
      />
    );
  }

  if (view === "delete") {
    return (
      <DeleteConfirm
        title={`Delete “${folder.name}”?`}
        message={
          kind === "collection"
            ? "This also deletes everything inside it: its categories and all their links. This can't be undone."
            : "This also deletes all the links inside it. This can't be undone."
        }
        onDelete={() => onDelete(folder.id)}
        onBack={() => setView("menu")}
        onDone={onChanged}
      />
    );
  }

  return (
    <>
      <SheetHeader title={folder.name} onClose={onClose} />
      <ActionRow
        icon="edit-2"
        label="Rename"
        onPress={() => setView("rename")}
      />
      <ActionRow
        icon="trash-2"
        label="Delete"
        onPress={() => setView("delete")}
      />
    </>
  );
}
