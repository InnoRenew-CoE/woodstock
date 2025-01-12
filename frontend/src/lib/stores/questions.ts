import type { Question } from "$lib/types/question";
import { get, writable, type Writable } from "svelte/store";

export const questionsStore: Writable<Question[]> = writable([
  { level: 0, title: "File upload" },
  { level: 0, title: "Submission" },
]);

export function findParentQuestionIndex(question: Question): { min: number; max: number } {
  const questions = get(questionsStore);
  const currentIndex = questions.indexOf(question);
  let min = 0;
  let max = 0;
  for (let i = currentIndex; i > 0; i--) {
    if (questions[i]?.level < question.level) {
      min = i;
      break;
    }
  }
  for (let i = currentIndex; i < questions.length; i++) {
    if (questions[i]?.level < question.level) {
      max = i;
      break;
    }
  }
  return { min: min, max: max };
}
