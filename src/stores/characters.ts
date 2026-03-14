import { computed, readonly, shallowRef } from "vue";
import { defineStore } from "pinia";
import { timeOperation } from "../utils/timeOp";
import { invoke } from "@tauri-apps/api/core";

export interface Character {
  id: string;
  character: string;
  costume: string;
  character_image: string;
  character_skill_preview?: string;
  standing?: boolean;
  cutscene?: boolean;
  dating_id?: string;
  release_date?: string;
  is_collab?: boolean;
  gender?: string;
}

interface CharactersJson {
  characters: Character[];
  dating: DatingMap;
}

type DatingMap = Record<string, string>;

export const useCharactersStore = defineStore("characters", () => {
  const characters = shallowRef<Character[]>([]);
  const datingMap = shallowRef<DatingMap>({});
  const charactersMap = shallowRef<Record<string, Character>>({});
  const datingToCharacterMap = shallowRef<Record<string, Character>>({});

  const groupedCharacters = computed<Record<string, Character[]>>(() => {
    return characters.value.reduce((acc, c) => {
      if (!acc[c.character]) {
        acc[c.character] = [];
      }
      acc[c.character].push(c);
      return acc;
    }, {} as Record<string, Character[]>);
  });

  function updateCharacters(data: CharactersJson) {
    timeOperation(() => {
      characters.value = data.characters
      datingMap.value = data.dating;

      charactersMap.value = Object.fromEntries(
        data.characters.map((c) => [c.id, c])
      );

      datingToCharacterMap.value = Object.fromEntries(
        Object.entries(data.dating)
          .map(([key, id]) => {
            const character = getCharacterById(id);
            return character ? [key, character] : null;
          })
          .filter((entry): entry is [string, Character] => entry !== null)
      );
    }, `updateCharacters (${data.characters.length} characters, ${Object.keys(data.dating).length} dating mappings)`);
  }

  function getCharacterById(id: string): Character | null {
    return timeOperation(() => {
      return charactersMap.value[id] || null;
    }, `getCharacterById(${id})`, false);
  }

  function getCharacterByDatingId(datingId: string): Character | null {
    return timeOperation(() => {
      return datingToCharacterMap.value[datingId] || null;
    }, `getCharacterByDatingId(${datingId})`, false);
  }

  function getCharacterIdByDatingId(datingId: string): string | null {
    return timeOperation(() => {
      return datingMap.value[datingId] || null;
    }, `getCharacterIdByDatingId(${datingId})`, false);
  }

async function loadCharacters() {
  try {
    const data = await invoke<CharactersJson>("get_characters");
    updateCharacters(data);
    if (import.meta.env.DEV)
      console.info("Local characters loaded:", characters.value.length);
  } catch (error) {
    console.error("Failed to load local characters:", error);
  }
}

  return {
    characters: readonly(characters),
    groupedCharacters: readonly(groupedCharacters),
    loadCharacters,
    getCharacterById,
    getCharacterByDatingId,
    getCharacterIdByDatingId,
  };
});