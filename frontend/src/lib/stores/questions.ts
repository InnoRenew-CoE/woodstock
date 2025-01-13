import { QuestionType, type Answer, type Question } from "$lib/types/question";
import { get, writable, type Writable } from "svelte/store";

export const answersStore: Writable<Answer[]> = writable([]);
export const filesStore: Writable<string[]> = writable(["research.pdf", "hello.txt", "rat.exe", "trojan.sh"]);
export const questionsStore: Writable<Question[]> = writable([
  {
    id: 4,
    title: "Description",
    possible_answers: [],
    text: "Please describe the file in a few short sentences.",
    type: QuestionType.Text,
  },
  {
    id: 1,
    title: "Condifentiality",
    possible_answers: [
      { id: 0, value: "Public" },
      { id: 1, value: "Internal only" },
      { id: 2, value: "Restricted" },
    ],
    text: "",
    type: QuestionType.Select,
  },
  {
    id: 0,
    title: "Primary purpose",
    possible_answers: [
      { id: 0, value: "Informational" },
      { id: 1, value: "Instructional" },
      { id: 2, value: "Analytical" },
      { id: 3, value: "Educational" },
      { id: 4, value: "Technical Reference" },
    ],
    text: "",
    type: QuestionType.MultiSelect,
  },
  {
    id: 2,
    title: "Topics",
    possible_answers: [
      { id: 1, value: "Marketing" },
      { id: 2, value: "Finance" },
      { id: 3, value: "Technology" },
      { id: 4, value: "Human Resources" },
      { id: 5, value: "Education" },
      { id: 6, value: "Design" },
    ],
    text: "What topics or categories does this file belong to?",
    type: QuestionType.MultiSelect,
  },
]);
