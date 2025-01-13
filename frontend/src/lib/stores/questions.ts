import { QuestionType, type Answer, type Question } from "$lib/types/question";
import { get, writable, type Writable } from "svelte/store";

export const answersStore: Writable<Answer[]> = writable([]);
export const filesStore: Writable<FileList | undefined> = writable();
export const questionsStore: Writable<Question[]> = writable([
  {
    id: 4,
    title: "Description",
    possible_answers: [],
    answers: [],
    text: "Please describe the file in a few short sentences.",
    type: QuestionType.Text,
  },
  {
    id: 1,
    title: "Confidentiality",
    possible_answers: [
      { id: 0, value: "Public" },
      { id: 1, value: "Internal only" },
      { id: 2, value: "Restricted" },
    ],
    answers: [],
    text: "Please specify how you'd like this file to be considered in the system.",
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
    answers: [],
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
    answers: [],
    text: "What topics or categories does this file belong to?",
    type: QuestionType.MultiSelect,
  },
]);
