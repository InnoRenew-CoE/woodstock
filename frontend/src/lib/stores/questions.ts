import { PUBLIC_API_BASE_URL } from "$env/static/public";
import { QuestionType, type Answer, type FileAnswer, type Question } from "$lib/types/question";
import { get, writable, type Writable } from "svelte/store";
import { pushNotification } from "./notifications";

export const answersStore: Writable<Answer[]> = writable([]);
export const filesStore: Writable<FileList | undefined> = writable();
export const questionsStore: Writable<Question[]> = writable([]);

export async function fetchQuestions() {
  const request = await fetch(`${PUBLIC_API_BASE_URL}/api/questions`);
  const json = await request.json();
  const questions: Question[] = JSON.parse(json);
  questionsStore.set(questions);
}

export async function submitAnswers(files: FileList) {
  const data = [
    {
      file: "CustomGauge.swift",
      answers: [
        { question_id: 1, text: "AET", selection: [] },
        { question_id: 2, text: "", selection: [2] },
      ],
    },
    {
      file: "LogListView.swift",
      answers: [
        { question_id: 1, text: "AE", selection: [] },
        { question_id: 2, text: "", selection: [1] },
      ],
    },
    {
      file: "StatisticsView.swift",
      answers: [
        { question_id: 1, text: "AER", selection: [] },
        { question_id: 2, text: "", selection: [3] },
      ],
    },
    {
      file: "TripListView.swift",
      answers: [
        { question_id: 1, text: "SA", selection: [] },
        { question_id: 2, text: "", selection: [2] },
      ],
    },
  ];

  const formData = new FormData();
  formData.append("answers", "[]");
  for (let x of files) {
    formData.append(x.name, x);
  }

  const response = await fetch(`${PUBLIC_API_BASE_URL}/api/answers`, {
    method: "POST",
    body: formData,
  });

  pushNotification({ title: "Success", body: "Submission successful." });
}
