import { QuestionType, type Question } from "$lib/types/question";
import { get, writable, type Writable } from "svelte/store";

export const filesStore: Writable<string[]> = writable(["research.pdf", "hello.txt", "rat.exe", "trojan.sh"]);
export const questionsStore: Writable<Question[]> = writable([
  { id: 0, title: "A", possible_answers: [], text: "", type: QuestionType.Select },
  { id: 1, title: "B", possible_answers: [], text: "", type: QuestionType.Select },
  { id: 2, title: "C", possible_answers: [], text: "", type: QuestionType.Select },
]);
