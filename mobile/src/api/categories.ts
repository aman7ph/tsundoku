import { api } from "@/api/client";

export type Category = {
  id: string;
  section_id: string;
  name: string;
  icon: string | null;
  position: number;
  created_at: string;
};

export async function listCategories(sectionId: string): Promise<Category[]> {
  const { data } = await api.get<Category[]>(
    `/sections/${sectionId}/categories`,
  );
  return data;
}

export async function createCategory(
  sectionId: string,
  name: string,
): Promise<Category> {
  const { data } = await api.post<Category>(
    `/sections/${sectionId}/categories`,
    { name },
  );
  return data;
}

export async function renameCategory(
  id: string,
  name: string,
): Promise<Category> {
  const { data } = await api.patch<Category>(`/categories/${id}`, { name });
  return data;
}

/** Deletes the category with its links */
export async function deleteCategory(id: string): Promise<void> {
  await api.delete(`/categories/${id}`);
}
