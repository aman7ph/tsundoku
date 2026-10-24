import { api } from "@/api/client";

export type Section = {
  id: string;
  name: string;
  icon: string | null;
  position: number;
  created_at: string;
};

export async function listSections(): Promise<Section[]> {
  const { data } = await api.get<Section[]>("/sections");
  return data;
}

export async function createSection(name: string): Promise<Section> {
  const { data } = await api.post<Section>("/sections", { name });
  return data;
}
